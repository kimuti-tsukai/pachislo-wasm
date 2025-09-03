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

#[wasm_bindgen]
#[derive(Default)]
pub struct JsInput;

#[wasm_bindgen]
impl JsInput {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        JsInput
    }
}

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

#[wasm_bindgen]
pub enum ControlFlow {
    Continue,
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

type InnerGame = Game<JsInput, JsOutput, Box<dyn FnMut(usize) -> f64>>;

#[wasm_bindgen]
pub struct WasmGame {
    game: Mutex<InnerGame>,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new(input: JsInput, output: JsOutput, config: Config) -> Self {
        Self {
            game: Mutex::new(Game::new(config.into(), input, output).unwrap()),
        }
    }

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
