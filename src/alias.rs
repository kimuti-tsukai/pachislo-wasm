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

#[cfg(test)]
mod tests {
    use super::*;

    // Only test functions that don't require WebAssembly bindings
    // WebAssembly-specific tests should be run with wasm-pack test

    #[test]
    fn test_transition_creation() {
        use pachislo::game::GameState as PachisloGameState;
        use pachislo::game::Transition as PachisloTransition;

        let before_state = PachisloGameState::Normal { balls: 50 };
        let after_state = PachisloGameState::Rush {
            balls: 60,
            rush_balls: 10,
            n: 1,
        };

        let pachislo_transition = PachisloTransition {
            before: Some(before_state),
            after: after_state,
        };

        let transition = Transition::from(pachislo_transition);

        assert!(transition.before.is_some());
        match transition.before.unwrap() {
            GameState::Normal { balls } => assert_eq!(balls, 50),
            _ => panic!("Expected Normal state"),
        }

        match transition.after {
            GameState::Rush {
                balls,
                rush_balls,
                n,
            } => {
                assert_eq!(balls, 60);
                assert_eq!(rush_balls, 10);
                assert_eq!(n, 1);
            }
            _ => panic!("Expected Rush state"),
        }
    }

    #[test]
    fn test_game_state_conversions() {
        use pachislo::game::GameState as PachisloGameState;

        // Test Uninitialized
        let uninitialized = PachisloGameState::Uninitialized;
        let converted = GameState::from(uninitialized);
        assert!(matches!(converted, GameState::Uninitialized));

        // Test Normal
        let normal = PachisloGameState::Normal { balls: 100 };
        let converted = GameState::from(normal);
        match converted {
            GameState::Normal { balls } => assert_eq!(balls, 100),
            _ => panic!("Expected Normal state"),
        }

        // Test Rush
        let rush = PachisloGameState::Rush {
            balls: 150,
            rush_balls: 25,
            n: 3,
        };
        let converted = GameState::from(rush);
        match converted {
            GameState::Rush {
                balls,
                rush_balls,
                n,
            } => {
                assert_eq!(balls, 150);
                assert_eq!(rush_balls, 25);
                assert_eq!(n, 3);
            }
            _ => panic!("Expected Rush state"),
        }
    }

    #[test]
    fn test_lottery_result_conversions() {
        use pachislo::lottery::{
            Lose as PachisloLose, LotteryResult as PachisloLotteryResult, Win as PachisloWin,
        };

        // Test Win conversions
        let pachislo_default_win = PachisloLotteryResult::Win(PachisloWin::Default);
        let converted_win = LotteryResult::from(pachislo_default_win);
        match converted_win {
            LotteryResult::Win(Win::Default) => assert!(true),
            _ => panic!("Expected Win::Default"),
        }

        let pachislo_fake_win = PachisloLotteryResult::Win(PachisloWin::FakeWin);
        let converted_fake_win = LotteryResult::from(pachislo_fake_win);
        match converted_fake_win {
            LotteryResult::Win(Win::FakeWin) => assert!(true),
            _ => panic!("Expected Win::FakeWin"),
        }

        // Test Lose conversions
        let pachislo_default_lose = PachisloLotteryResult::Lose(PachisloLose::Default);
        let converted_lose = LotteryResult::from(pachislo_default_lose);
        match converted_lose {
            LotteryResult::Lose(Lose::Default) => assert!(true),
            _ => panic!("Expected Lose::Default"),
        }

        let pachislo_fake_lose = PachisloLotteryResult::Lose(PachisloLose::FakeLose);
        let converted_fake_lose = LotteryResult::from(pachislo_fake_lose);
        match converted_fake_lose {
            LotteryResult::Lose(Lose::FakeLose) => assert!(true),
            _ => panic!("Expected Lose::FakeLose"),
        }
    }

    #[test]
    fn test_lottery_result_is_win() {
        let win_result = LotteryResult::Win(Win::Default);
        assert!(win_result.is_win());

        let fake_win_result = LotteryResult::Win(Win::FakeWin);
        assert!(fake_win_result.is_win());

        let lose_result = LotteryResult::Lose(Lose::Default);
        assert!(!lose_result.is_win());

        let fake_lose_result = LotteryResult::Lose(Lose::FakeLose);
        assert!(!fake_lose_result.is_win());
    }

    #[test]
    fn test_slot_probability_creation() {
        let prob = SlotProbability::new(0.1, 0.05, 0.02);
        assert_eq!(prob.win, 0.1);
        assert_eq!(prob.fake_win, 0.05);
        assert_eq!(prob.fake_lose, 0.02);
    }

    #[test]
    fn test_slot_probability_conversion() {
        let slot_prob = SlotProbability::new(0.15, 0.08, 0.03);
        let pachislo_prob: pachislo::config::SlotProbability = slot_prob.into();

        assert_eq!(pachislo_prob.win, 0.15);
        assert_eq!(pachislo_prob.fake_win, 0.08);
        assert_eq!(pachislo_prob.fake_lose, 0.03);
    }

    #[test]
    fn test_balls_config_creation() {
        let config = BallsConfig::new(100, 15, 50);
        assert_eq!(config.init_balls, 100);
        assert_eq!(config.incremental_balls, 15);
        assert_eq!(config.incremental_rush, 50);
    }

    #[test]
    fn test_balls_config_conversion() {
        let balls_config = BallsConfig::new(200, 20, 80);
        let pachislo_config: pachislo::config::BallsConfig = balls_config.into();

        assert_eq!(pachislo_config.init_balls, 200);
        assert_eq!(pachislo_config.incremental_balls, 20);
        assert_eq!(pachislo_config.incremental_rush, 80);
    }

    // WebAssembly-specific tests (Probability, Config creation/conversion) are disabled for non-WASM targets
    // These should be run using `wasm-pack test` in a browser environment

    #[test]
    fn test_probability_edge_cases() {
        // Test with zero probabilities
        let zero_prob = SlotProbability::new(0.0, 0.0, 0.0);
        assert_eq!(zero_prob.win, 0.0);
        assert_eq!(zero_prob.fake_win, 0.0);
        assert_eq!(zero_prob.fake_lose, 0.0);

        // Test with maximum probabilities
        let max_prob = SlotProbability::new(1.0, 1.0, 1.0);
        assert_eq!(max_prob.win, 1.0);
        assert_eq!(max_prob.fake_win, 1.0);
        assert_eq!(max_prob.fake_lose, 1.0);
    }

    #[test]
    fn test_balls_config_edge_cases() {
        // Test with zero balls
        let zero_config = BallsConfig::new(0, 0, 0);
        assert_eq!(zero_config.init_balls, 0);
        assert_eq!(zero_config.incremental_balls, 0);
        assert_eq!(zero_config.incremental_rush, 0);

        // Test with large numbers
        let large_config = BallsConfig::new(usize::MAX, usize::MAX, usize::MAX);
        assert_eq!(large_config.init_balls, usize::MAX);
        assert_eq!(large_config.incremental_balls, usize::MAX);
        assert_eq!(large_config.incremental_rush, usize::MAX);
    }

    #[test]
    fn test_win_lose_enum_variants() {
        // Test Win variants
        let default_win = Win::Default;
        let fake_win = Win::FakeWin;

        // Test Lose variants
        let default_lose = Lose::Default;
        let fake_lose = Lose::FakeLose;

        // Test that they can be used in match statements
        match default_win {
            Win::Default => assert!(true),
            Win::FakeWin => panic!("Should be Default"),
        }

        match fake_win {
            Win::Default => panic!("Should be FakeWin"),
            Win::FakeWin => assert!(true),
        }

        match default_lose {
            Lose::Default => assert!(true),
            Lose::FakeLose => panic!("Should be Default"),
        }

        match fake_lose {
            Lose::Default => panic!("Should be FakeLose"),
            Lose::FakeLose => assert!(true),
        }
    }
}
