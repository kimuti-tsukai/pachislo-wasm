//! # Type Aliases and JavaScript Bindings
//!
//! This module contains type aliases and structures that provide JavaScript-compatible
//! interfaces for the pachislo game engine. All types in this module are designed
//! to be serializable to/from JavaScript using wasm-bindgen and serde.

use js_sys::Function;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

/// Represents a state transition in the pachislo game.
///
/// This structure captures a change from one game state to another,
/// including both the previous state (if any) and the new state.
/// It is automatically serialized to JavaScript objects.
///
/// # Fields
///
/// * `before` - The previous game state, `None` if this is the initial state
/// * `after` - The new game state after the transition
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Transition {
    pub before: Option<GameState>,
    pub after: GameState,
}

impl From<pachislo::game::Transition> for Transition {
    fn from(transition: pachislo::game::Transition) -> Self {
        Transition {
            before: transition.before.map(|state| state.into()),
            after: transition.after.into(),
        }
    }
}

/// Represents the current state of a pachislo game session.
///
/// The game can be in one of three states:
/// - `Uninitialized`: Game hasn't started yet
/// - `Normal`: Standard gameplay mode
/// - `Rush`: Special high-probability bonus mode
///
/// # Variants
///
/// * `Uninitialized` - Initial state before game starts
/// * `Normal { balls }` - Standard mode with current ball count
/// * `Rush { balls, rush_balls, n }` - Rush mode with ball counts and continuation counter
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum GameState {
    /// Game has not been initialized yet
    Uninitialized,
    /// Normal gameplay mode
    Normal {
        /// Current number of balls the player has
        balls: usize,
    },
    /// Rush mode with enhanced winning probabilities
    Rush {
        /// Current number of regular balls
        balls: usize,
        /// Number of special rush balls
        rush_balls: usize,
        /// Rush continuation counter
        n: usize,
    },
}

impl From<pachislo::game::GameState> for GameState {
    fn from(state: pachislo::game::GameState) -> Self {
        match state {
            pachislo::game::GameState::Uninitialized => GameState::Uninitialized,
            pachislo::game::GameState::Normal { balls } => GameState::Normal { balls },
            pachislo::game::GameState::Rush {
                balls,
                rush_balls,
                n,
            } => GameState::Rush {
                balls,
                rush_balls,
                n,
            },
        }
    }
}

/// Represents the result of a lottery draw in the pachislo game.
///
/// Each lottery can result in either a win or a loss, with different
/// subtypes for each outcome that may affect game behavior differently.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum LotteryResult {
    /// A winning lottery result
    Win(Win),
    /// A losing lottery result
    Lose(Lose),
}

/// Types of winning lottery results.
///
/// Different win types may trigger different animations, sounds,
/// or game behaviors while still being treated as wins.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
pub enum Win {
    /// Standard winning result
    Default,
    /// A win that appears as a loss initially but reveals as a win (surprise element)
    FakeWin,
}

/// Types of losing lottery results.
///
/// Different lose types may trigger different animations, sounds,
/// or game behaviors while still being treated as losses.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
pub enum Lose {
    /// Standard losing result
    Default,
    /// A loss that appears as a win initially but reveals as a loss (near-miss element)
    FakeLose,
}

impl From<pachislo::lottery::Win> for Win {
    fn from(win: pachislo::lottery::Win) -> Self {
        match win {
            pachislo::lottery::Win::Default => Win::Default,
            pachislo::lottery::Win::FakeWin => Win::FakeWin,
        }
    }
}

impl From<pachislo::lottery::Lose> for Lose {
    fn from(lose: pachislo::lottery::Lose) -> Self {
        match lose {
            pachislo::lottery::Lose::Default => Lose::Default,
            pachislo::lottery::Lose::FakeLose => Lose::FakeLose,
        }
    }
}

impl From<pachislo::lottery::LotteryResult> for LotteryResult {
    fn from(result: pachislo::lottery::LotteryResult) -> Self {
        match result {
            pachislo::lottery::LotteryResult::Win(win) => LotteryResult::Win(win.into()),
            pachislo::lottery::LotteryResult::Lose(lose) => LotteryResult::Lose(lose.into()),
        }
    }
}

#[wasm_bindgen]
impl LotteryResult {
    /// Checks if the lottery result is a win.
    ///
    /// # Returns
    ///
    /// `true` if the result is any type of win, `false` otherwise.
    #[wasm_bindgen]
    pub fn is_win(&self) -> bool {
        matches!(self, LotteryResult::Win(_))
    }
}

/// Main configuration structure for the pachislo game.
///
/// This structure contains all the settings needed to configure
/// a pachislo game session, including ball counts and probability
/// settings for different game modes.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Config {
    /// Ball-related configuration settings
    pub balls: BallsConfig,
    /// Probability settings for different game modes
    probability: Probability,
}

/// Configuration for ball-related game mechanics.
///
/// This structure defines how many balls players start with
/// and how many they gain in different situations.
#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct BallsConfig {
    /// Number of balls the player starts with
    pub init_balls: usize,
    /// Number of balls gained for normal wins
    pub incremental_balls: usize,
    /// Number of balls gained when entering rush mode
    pub incremental_rush: usize,
}

/// Probability settings for slot machine outcomes.
///
/// These probabilities determine the likelihood of different
/// lottery results during gameplay. All probabilities should
/// be values between 0.0 and 1.0.
#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct SlotProbability {
    /// Probability of a regular win (0.0 to 1.0)
    pub win: f64,
    /// Probability of a fake win result (0.0 to 1.0)
    pub fake_win: f64,
    /// Probability of a fake lose result (0.0 to 1.0)
    pub fake_lose: f64,
}

/// Complete probability configuration for all game modes.
///
/// This structure contains probability settings for each game mode
/// and a function to calculate rush continuation probability.
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Probability {
    /// Probabilities during normal mode
    pub normal: SlotProbability,
    /// Probabilities during rush mode
    pub rush: SlotProbability,
    /// Probabilities for rush continuation
    pub rush_continue: SlotProbability,
    /// JavaScript function that calculates rush continuation probability based on current count
    rush_continue_fn: Function,
}

impl From<SlotProbability> for pachislo::config::SlotProbability {
    fn from(probability: SlotProbability) -> Self {
        pachislo::config::SlotProbability {
            win: probability.win,
            fake_win: probability.fake_win,
            fake_lose: probability.fake_lose,
        }
    }
}

#[wasm_bindgen]
impl SlotProbability {
    /// Creates a new SlotProbability configuration.
    ///
    /// # Arguments
    ///
    /// * `win` - Probability of regular wins (0.0 to 1.0)
    /// * `fake_win` - Probability of fake wins (0.0 to 1.0)
    /// * `fake_lose` - Probability of fake losses (0.0 to 1.0)
    ///
    /// # Note
    ///
    /// The sum of all probabilities doesn't need to equal 1.0 as they
    /// are applied in a specific order by the game engine.
    #[wasm_bindgen(constructor)]
    pub fn new(win: f64, fake_win: f64, fake_lose: f64) -> Self {
        SlotProbability {
            win,
            fake_win,
            fake_lose,
        }
    }
}

#[wasm_bindgen]
impl Probability {
    /// Creates a new Probability configuration.
    ///
    /// # Arguments
    ///
    /// * `normal` - Probability settings for normal mode
    /// * `rush` - Probability settings for rush mode
    /// * `rush_continue` - Probability settings for rush continuation
    /// * `rush_continue_fn` - JavaScript function that takes a number (current rush count)
    ///   and returns the probability of continuing the rush
    ///
    /// # Example JavaScript Function
    ///
    /// ```javascript
    /// const rushContinueFn = (n) => Math.max(0.1, 0.8 - n * 0.1);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(
        normal: SlotProbability,
        rush: SlotProbability,
        rush_continue: SlotProbability,
        rush_continue_fn: Function,
    ) -> Self {
        Probability {
            normal,
            rush,
            rush_continue,
            rush_continue_fn,
        }
    }
}

impl From<Probability> for pachislo::config::Probability<Box<dyn FnMut(usize) -> f64>> {
    fn from(probability: Probability) -> Self {
        pachislo::config::Probability {
            normal: probability.normal.into(),
            rush: probability.rush.into(),
            rush_continue: probability.rush_continue.into(),
            rush_continue_fn: Box::new(move |n| {
                probability
                    .rush_continue_fn
                    .call1(&JsValue::NULL, &JsValue::from(n))
                    .unwrap()
                    .as_f64()
                    .unwrap()
            }),
        }
    }
}

impl From<BallsConfig> for pachislo::config::BallsConfig {
    fn from(config: BallsConfig) -> Self {
        pachislo::config::BallsConfig {
            init_balls: config.init_balls,
            incremental_balls: config.incremental_balls,
            incremental_rush: config.incremental_rush,
        }
    }
}

#[wasm_bindgen]
impl BallsConfig {
    /// Creates a new BallsConfig.
    ///
    /// # Arguments
    ///
    /// * `init_balls` - Initial number of balls when the game starts
    /// * `incremental_balls` - Balls gained on normal wins
    /// * `incremental_rush` - Balls gained when entering rush mode
    ///
    /// # Example
    ///
    /// ```javascript
    /// const ballsConfig = new BallsConfig(100, 15, 50);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(init_balls: usize, incremental_balls: usize, incremental_rush: usize) -> Self {
        BallsConfig {
            init_balls,
            incremental_balls,
            incremental_rush,
        }
    }
}

#[wasm_bindgen]
impl Config {
    /// Creates a new game configuration.
    ///
    /// # Arguments
    ///
    /// * `balls` - Ball-related configuration
    /// * `probability` - Probability settings for all game modes
    ///
    /// # Returns
    ///
    /// A complete configuration ready to be used with WasmGame.
    #[wasm_bindgen(constructor)]
    pub fn new(balls: BallsConfig, probability: Probability) -> Self {
        Config { balls, probability }
    }
}

impl From<Config> for pachislo::config::Config<Box<dyn FnMut(usize) -> f64>> {
    fn from(config: Config) -> Self {
        pachislo::config::Config {
            balls: config.balls.into(),
            probability: config.probability.into(),
        }
    }
}
