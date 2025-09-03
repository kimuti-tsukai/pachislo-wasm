//! # Pachislo WASM
//!
//! A WebAssembly binding for the pachislo (Japanese slot machine) game library.
//! This crate provides JavaScript-compatible interfaces for running pachislo games
//! in web browsers and Node.js environments.
//!
//! ## Features
//!
//! - Complete pachislo game simulation with normal and rush modes
//! - JavaScript-friendly API with automatic serialization
//! - Real-time game state transitions
//! - Customizable probability settings and ball configurations
//! - Slot machine visual representation generation
//!
//! ## Usage
//!
//! ```javascript
//! import { WasmGame, JsInput, JsOutput, Config, BallsConfig, Probability, SlotProbability } from 'pachislo-wasm';
//!
//! // Configure the game
//! const ballsConfig = new BallsConfig(100, 10, 5);
//! const normalProb = new SlotProbability(0.1, 0.05, 0.02);
//! const rushProb = new SlotProbability(0.8, 0.1, 0.05);
//! const rushContinueProb = new SlotProbability(0.7, 0.1, 0.05);
//! const probability = new Probability(normalProb, rushProb, rushContinueProb, (n) => 0.5);
//! const config = new Config(ballsConfig, probability);
//!
//! // Set up input/output handlers
//! const input = new JsInput();
//! const output = new JsOutput(context, defaultHandler, finishHandler, normalLotteryHandler, rushLotteryHandler, rushContinueHandler);
//!
//! // Create and run the game
//! const game = new WasmGame(input, output, config);
//! game.run_step_with_command("StartGame");
//! ```

use std::sync::Mutex;

use js_sys::Function;
use pachislo::{
    Game,
    command::{CauseLottery, Command, FinishGame, LaunchBall, StartGame},
    interface::{UserInput, UserOutput},
    slot::SlotProducer,
};
use rand::Rng;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::alias::{Config, GameState, LotteryResult, Transition};

pub mod alias;

/// Converts a string command to a pachislo Command enum.
///
/// # Arguments
///
/// * `input` - The string representation of the command
///
/// # Returns
///
/// Returns `Some(Command)` if the input string matches a valid command,
/// otherwise returns `None`.
///
/// # Supported Commands
///
/// - `"LaunchBall"` - Launch a ball into the machine
/// - `"CauseLottery"` - Trigger the lottery mechanism
/// - `"StartGame"` - Start a new game session
/// - `"FinishGame"` - End the current game session
/// - `"Finish"` - Alias for finishing the game
fn convert_string_to_command<F, R>(input: &str) -> Option<Command<JsInput, JsOutput, F, R>>
where
    F: FnMut(usize) -> f64,
    R: Rng,
{
    match input {
        "LaunchBall" => Some(Command::control(LaunchBall)),
        "CauseLottery" => Some(Command::control(CauseLottery)),
        "StartGame" => Some(Command::control(StartGame)),
        "FinishGame" => Some(Command::control(FinishGame)),
        "Finish" => Some(Command::FinishGame),
        _ => None,
    }
}

/// JavaScript-compatible input handler for the pachislo game.
///
/// This struct implements the `UserInput` trait and serves as a bridge
/// between JavaScript and the Rust pachislo game engine. Currently,
/// it provides a placeholder implementation as input is handled
/// through the command system.
#[wasm_bindgen]
#[derive(Default)]
pub struct JsInput;

#[wasm_bindgen]
impl JsInput {
    /// Creates a new instance of `JsInput`.
    ///
    /// # Returns
    ///
    /// A new `JsInput` instance ready to be used with the game engine.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        JsInput
    }
}

/// JavaScript-compatible output handler for the pachislo game.
///
/// This struct implements the `UserOutput` trait and manages all
/// communication from the Rust game engine back to JavaScript.
/// It holds references to JavaScript callback functions and handles
/// the serialization of game state data.
///
/// # Fields
///
/// - `context` - JavaScript context object passed to callback functions
/// - `default` - Callback for general state transitions
/// - `finish_game` - Callback when the game session ends
/// - `lottery_normal` - Callback for normal mode lottery results
/// - `lottery_rush` - Callback for rush mode lottery results
/// - `lottery_rush_continue` - Callback for rush continuation lottery results
/// - `slot_producer` - Generates visual slot machine representations
#[wasm_bindgen]
pub struct JsOutput {
    context: JsValue,
    default: Function,
    finish_game: Function,
    lottery_normal: Function,
    lottery_rush: Function,
    lottery_rush_continue: Function,
    slot_producer: SlotProducer<u8>,
}

#[wasm_bindgen]
impl JsOutput {
    /// Creates a new instance of `JsOutput` with JavaScript callback functions.
    ///
    /// # Arguments
    ///
    /// * `context` - JavaScript context object to be passed to all callbacks
    /// * `default` - Callback function for general game state transitions
    /// * `finish_game` - Callback function called when the game ends
    /// * `lottery_normal` - Callback function for normal mode lottery results
    /// * `lottery_rush` - Callback function for rush mode lottery results
    /// * `lottery_rush_continue` - Callback function for rush continuation results
    ///
    /// # Returns
    ///
    /// A new `JsOutput` instance configured with the provided callbacks.
    /// The slot producer is automatically initialized with 3 reels and symbols 1-7.
    #[wasm_bindgen(constructor)]
    pub fn new(
        context: JsValue,
        default: Function,
        finish_game: Function,
        lottery_normal: Function,
        lottery_rush: Function,
        lottery_rush_continue: Function,
    ) -> Self {
        JsOutput {
            context,
            default,
            finish_game,
            lottery_normal,
            lottery_rush,
            lottery_rush_continue,
            slot_producer: SlotProducer::new(3, (1..=7).collect()),
        }
    }
}

impl<F, R> UserInput<JsOutput, F, R> for JsInput
where
    F: FnMut(usize) -> f64,
    R: Rng,
{
    fn wait_for_input(&mut self) -> Command<Self, JsOutput, F, R> {
        unreachable!()
    }
}

impl UserOutput for JsOutput {
    fn default(&mut self, state: pachislo::game::Transition) {
        self.default
            .call1(
                &self.context,
                &serde_wasm_bindgen::to_value(&Transition::from(state)).unwrap(),
            )
            .unwrap();
    }

    fn finish_game(&mut self, state: &pachislo::game::GameState) {
        self.finish_game
            .call1(
                &self.context,
                &serde_wasm_bindgen::to_value(&GameState::from(*state)).unwrap(),
            )
            .unwrap();
    }

    fn lottery_normal(&mut self, result: pachislo::lottery::LotteryResult) {
        let slot = self.slot_producer.produce(&result);

        self.lottery_normal
            .call2(
                &self.context,
                &serde_wasm_bindgen::to_value(&LotteryResult::from(result)).unwrap(),
                &serde_wasm_bindgen::to_value(&slot).unwrap(),
            )
            .unwrap();
    }

    fn lottery_rush(&mut self, result: pachislo::lottery::LotteryResult) {
        let slot = self.slot_producer.produce(&result);

        self.lottery_rush
            .call2(
                &self.context,
                &serde_wasm_bindgen::to_value(&LotteryResult::from(result)).unwrap(),
                &serde_wasm_bindgen::to_value(&slot).unwrap(),
            )
            .unwrap();
    }

    fn lottery_rush_continue(&mut self, result: pachislo::lottery::LotteryResult) {
        let slot = self.slot_producer.produce(&result);

        self.lottery_rush_continue
            .call2(
                &self.context,
                &serde_wasm_bindgen::to_value(&LotteryResult::from(result)).unwrap(),
                &serde_wasm_bindgen::to_value(&slot).unwrap(),
            )
            .unwrap();
    }
}

/// Represents the control flow state of the game execution.
///
/// This enum is used to communicate whether the game should continue
/// running or should break out of the execution loop.
#[wasm_bindgen]
pub enum ControlFlow {
    /// The game should continue to the next step
    Continue,
    /// The game should break out of the execution loop
    Break,
}

impl From<std::ops::ControlFlow<()>> for ControlFlow {
    fn from(control_flow: std::ops::ControlFlow<()>) -> Self {
        match control_flow {
            std::ops::ControlFlow::Continue(()) => ControlFlow::Continue,
            std::ops::ControlFlow::Break(()) => ControlFlow::Break,
        }
    }
}

/// Type alias for the internal game instance with specific type parameters.
/// This represents a pachislo game with JavaScript input/output and a boxed
/// function for rush continuation probability calculation.
type InnerGame = Game<JsInput, JsOutput, Box<dyn FnMut(usize) -> f64>>;

/// The main WebAssembly-compatible pachislo game interface.
///
/// This struct wraps the core pachislo game engine and provides a
/// thread-safe interface that can be called from JavaScript. The game
/// is protected by a mutex to ensure safe concurrent access.
///
/// # Thread Safety
///
/// The game instance is wrapped in a `Mutex` to provide thread safety
/// when accessed from JavaScript, which may call methods from different
/// contexts or web workers.
#[wasm_bindgen]
pub struct WasmGame {
    game: Mutex<InnerGame>,
}

#[wasm_bindgen]
impl WasmGame {
    /// Creates a new pachislo game instance.
    ///
    /// # Arguments
    ///
    /// * `input` - The JavaScript input handler
    /// * `output` - The JavaScript output handler with callback functions
    /// * `config` - Game configuration including ball settings and probabilities
    ///
    /// # Returns
    ///
    /// A new `WasmGame` instance ready to accept commands.
    ///
    /// # Panics
    ///
    /// Panics if the game initialization fails due to invalid configuration.
    #[wasm_bindgen(constructor)]
    pub fn new(input: JsInput, output: JsOutput, config: Config) -> Self {
        Self {
            game: Mutex::new(Game::new(config.into(), input, output).unwrap()),
        }
    }

    /// Executes a single game step with the specified command.
    ///
    /// # Arguments
    ///
    /// * `command` - String representation of the command to execute.
    ///   See [`convert_string_to_command`] for supported commands.
    ///
    /// # Returns
    ///
    /// Returns `ControlFlow::Continue` if the game should continue,
    /// or `ControlFlow::Break` if the game has finished.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The command string is not recognized
    /// - The game mutex cannot be acquired
    /// - The game engine encounters an internal error
    ///
    /// # Example
    ///
    /// ```javascript
    /// const result = game.run_step_with_command("StartGame");
    /// if (result === ControlFlow.Continue) {
    ///     // Game continues, ready for next command
    /// }
    /// ```
    #[wasm_bindgen]
    pub fn run_step_with_command(&self, command: String) -> ControlFlow {
        let command = convert_string_to_command(&command).unwrap();

        self.game
            .lock()
            .unwrap()
            .run_step_with_command(command)
            .into()
    }
}
