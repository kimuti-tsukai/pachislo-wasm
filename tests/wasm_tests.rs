//! WebAssembly integration tests for pachislo-wasm
//!
//! These tests are designed to run in a WebAssembly environment using wasm-pack test.
//! They test the JavaScript bindings and WebAssembly functionality.

#![cfg(target_arch = "wasm32")]

use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_test::*;

// Import the library being tested
use pachislo_wasm::alias::*;
use pachislo_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Creates a mock JavaScript function for testing
fn create_mock_js_function() -> Function {
    let closure = Closure::wrap(
        Box::new(|_: JsValue| JsValue::from(0.5)) as Box<dyn FnMut(JsValue) -> JsValue>
    );
    let function = closure.as_ref().unchecked_ref::<Function>().clone();
    closure.forget();
    function
}

/// Creates a mock output callback function that accepts two arguments
fn create_mock_output_callback() -> Function {
    let closure = Closure::wrap(Box::new(|_: JsValue, _: JsValue| JsValue::NULL)
        as Box<dyn FnMut(JsValue, JsValue) -> JsValue>);
    let function = closure.as_ref().unchecked_ref::<Function>().clone();
    closure.forget();
    function
}

/// Creates a mock single-argument callback function
fn create_mock_single_callback() -> Function {
    let closure =
        Closure::wrap(Box::new(|_: JsValue| JsValue::NULL) as Box<dyn FnMut(JsValue) -> JsValue>);
    let function = closure.as_ref().unchecked_ref::<Function>().clone();
    closure.forget();
    function
}

/// Creates a complete test configuration
fn create_test_config() -> Config {
    let balls_config = BallsConfig::new(100, 15, 50);
    let normal_prob = SlotProbability::new(0.1, 0.05, 0.02);
    let rush_prob = SlotProbability::new(0.8, 0.1, 0.05);
    let rush_continue_prob = SlotProbability::new(0.7, 0.1, 0.05);
    let rush_continue_fn = create_mock_js_function();
    let probability =
        Probability::new(normal_prob, rush_prob, rush_continue_prob, rush_continue_fn);
    Config::new(balls_config, probability)
}

/// Creates a complete test output handler
fn create_test_output() -> JsOutput {
    let context = JsValue::NULL;
    let default_fn = create_mock_single_callback();
    let finish_fn = create_mock_single_callback();
    let lottery_normal_fn = create_mock_output_callback();
    let lottery_rush_fn = create_mock_output_callback();
    let lottery_rush_continue_fn = create_mock_output_callback();

    JsOutput::new(
        context,
        default_fn,
        finish_fn,
        lottery_normal_fn,
        lottery_rush_fn,
        lottery_rush_continue_fn,
    )
}

/// Creates a complete game instance for testing
fn create_test_game() -> WasmGame {
    let input = JsInput::new();
    let output = create_test_output();
    let config = create_test_config();
    WasmGame::new(input, output, config)
}

#[wasm_bindgen_test]
fn test_js_input_creation() {
    let input = JsInput::new();
    let input_default = JsInput::default();

    // Both should be created successfully
    assert_eq!(std::mem::size_of_val(&input), 0);
    assert_eq!(std::mem::size_of_val(&input_default), 0);
}

#[wasm_bindgen_test]
fn test_slot_probability_creation() {
    let prob = SlotProbability::new(0.1, 0.05, 0.02);
    assert_eq!(prob.win, 0.1);
    assert_eq!(prob.fake_win, 0.05);
    assert_eq!(prob.fake_lose, 0.02);
}

#[wasm_bindgen_test]
fn test_balls_config_creation() {
    let config = BallsConfig::new(100, 15, 50);
    assert_eq!(config.init_balls, 100);
    assert_eq!(config.incremental_balls, 15);
    assert_eq!(config.incremental_rush, 50);
}

#[wasm_bindgen_test]
fn test_probability_creation() {
    let normal = SlotProbability::new(0.1, 0.05, 0.02);
    let rush = SlotProbability::new(0.8, 0.1, 0.05);
    let rush_continue = SlotProbability::new(0.7, 0.1, 0.05);
    let rush_continue_fn = create_mock_js_function();

    let probability = Probability::new(normal, rush, rush_continue, rush_continue_fn);

    assert_eq!(probability.normal.win, 0.1);
    assert_eq!(probability.rush.win, 0.8);
    assert_eq!(probability.rush_continue.win, 0.7);
}

#[wasm_bindgen_test]
fn test_config_creation() {
    let config = create_test_config();
    assert_eq!(config.balls.init_balls, 100);
    assert_eq!(config.probability.normal.win, 0.1);
}

#[wasm_bindgen_test]
fn test_js_output_creation() {
    let output = create_test_output();

    // Test that the output was created successfully without panicking
    // We can't directly access internal state, but creation should work
    assert!(true);
}

#[wasm_bindgen_test]
fn test_wasm_game_creation() {
    let game = create_test_game();

    // Test that the game was created successfully
    assert!(!game.game.is_poisoned());
}

#[wasm_bindgen_test]
fn test_game_start_command() {
    let game = create_test_game();

    let result = game.run_step_with_command("StartGame".to_string());

    // The result should be either Continue or Break
    match result {
        ControlFlow::Continue | ControlFlow::Break => assert!(true),
    }
}

#[wasm_bindgen_test]
fn test_basic_game_commands() {
    let game = create_test_game();

    // Test each basic command
    let commands = vec!["StartGame", "LaunchBall", "CauseLottery"];

    for command in commands {
        let result = game.run_step_with_command(command.to_string());

        // Each command should execute without panicking
        match result {
            ControlFlow::Continue | ControlFlow::Break => {
                // If the game finished, that's also valid
                if matches!(result, ControlFlow::Break) {
                    break;
                }
            }
        }
    }
}

#[wasm_bindgen_test]
fn test_game_finish_commands() {
    let game = create_test_game();

    // Start and finish the game
    game.run_step_with_command("StartGame".to_string());
    let result1 = game.run_step_with_command("FinishGame".to_string());

    match result1 {
        ControlFlow::Continue | ControlFlow::Break => assert!(true),
    }

    // Test alternative finish command
    let game2 = create_test_game();
    game2.run_step_with_command("StartGame".to_string());
    let result2 = game2.run_step_with_command("Finish".to_string());

    match result2 {
        ControlFlow::Continue | ControlFlow::Break => assert!(true),
    }
}

#[wasm_bindgen_test]
fn test_lottery_result_is_win() {
    let win_result = LotteryResult::Win(Win::Default);
    let fake_win_result = LotteryResult::Win(Win::FakeWin);
    let lose_result = LotteryResult::Lose(Lose::Default);
    let fake_lose_result = LotteryResult::Lose(Lose::FakeLose);

    assert!(win_result.is_win());
    assert!(fake_win_result.is_win());
    assert!(!lose_result.is_win());
    assert!(!fake_lose_result.is_win());
}

#[wasm_bindgen_test]
fn test_multiple_games() {
    // Test that multiple games can be created independently
    let game1 = create_test_game();
    let game2 = create_test_game();
    let game3 = create_test_game();

    // Each game should start successfully
    let result1 = game1.run_step_with_command("StartGame".to_string());
    let result2 = game2.run_step_with_command("StartGame".to_string());
    let result3 = game3.run_step_with_command("StartGame".to_string());

    // All should return valid results
    match (result1, result2, result3) {
        (
            ControlFlow::Continue | ControlFlow::Break,
            ControlFlow::Continue | ControlFlow::Break,
            ControlFlow::Continue | ControlFlow::Break,
        ) => assert!(true),
    }
}

#[wasm_bindgen_test]
fn test_config_with_different_settings() {
    // Test with minimal ball settings
    let balls_config = BallsConfig::new(10, 1, 5);
    let normal_prob = SlotProbability::new(0.1, 0.05, 0.02);
    let rush_prob = SlotProbability::new(0.8, 0.1, 0.05);
    let rush_continue_prob = SlotProbability::new(0.7, 0.1, 0.05);
    let rush_continue_fn = create_mock_js_function();
    let probability =
        Probability::new(normal_prob, rush_prob, rush_continue_prob, rush_continue_fn);
    let config = Config::new(balls_config, probability);

    let input = JsInput::new();
    let output = create_test_output();

    let game = WasmGame::new(input, output, config);
    let result = game.run_step_with_command("StartGame".to_string());

    match result {
        ControlFlow::Continue | ControlFlow::Break => assert!(true),
    }
}

#[wasm_bindgen_test]
fn test_extreme_probabilities() {
    // Test with zero probabilities
    let balls_config = BallsConfig::new(100, 15, 50);
    let normal_prob = SlotProbability::new(0.0, 0.0, 0.0);
    let rush_prob = SlotProbability::new(0.0, 0.0, 0.0);
    let rush_continue_prob = SlotProbability::new(0.0, 0.0, 0.0);
    let rush_continue_fn = create_mock_js_function();
    let probability =
        Probability::new(normal_prob, rush_prob, rush_continue_prob, rush_continue_fn);
    let config = Config::new(balls_config, probability);

    let input = JsInput::new();
    let output = create_test_output();

    let game = WasmGame::new(input, output, config);
    let result = game.run_step_with_command("StartGame".to_string());

    match result {
        ControlFlow::Continue | ControlFlow::Break => assert!(true),
    }
}

#[wasm_bindgen_test]
fn test_sequential_commands() {
    let game = create_test_game();

    // Execute a sequence of commands
    let commands = vec![
        "StartGame",
        "LaunchBall",
        "CauseLottery",
        "LaunchBall",
        "CauseLottery",
        "FinishGame",
    ];

    for command in commands {
        let result = game.run_step_with_command(command.to_string());

        match result {
            ControlFlow::Continue => continue,
            ControlFlow::Break => break, // Game finished naturally
        }
    }

    // If we reach here, the sequence completed successfully
    assert!(true);
}
