use js_sys::Function;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

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

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum GameState {
    Uninitialized,
    Normal {
        balls: usize,
    },
    Rush {
        balls: usize,
        rush_balls: usize,
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum LotteryResult {
    Win(Win),
    Lose(Lose),
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
pub enum Win {
    Default,
    FakeWin,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
pub enum Lose {
    Default,
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
    #[wasm_bindgen]
    pub fn is_win(&self) -> bool {
        matches!(self, LotteryResult::Win(_))
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Config {
    pub balls: BallsConfig,
    probability: Probability,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct BallsConfig {
    pub init_balls: usize,
    pub incremental_balls: usize,
    pub incremental_rush: usize,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct SlotProbability {
    pub win: f64,
    pub fake_win: f64,
    pub fake_lose: f64,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Probability {
    pub normal: SlotProbability,
    pub rush: SlotProbability,
    pub rush_continue: SlotProbability,
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
impl Probability {
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
impl Config {
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
