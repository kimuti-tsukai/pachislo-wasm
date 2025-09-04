# Pachislo WASM

A WebAssembly wrapper for the [pachislo](https://crates.io/crates/pachislo) Rust crate, providing a JavaScript/TypeScript interface for pachislo (Japanese slot machine) game simulation.

## Overview

This library allows you to run pachislo game simulations in web browsers or Node.js environments by compiling Rust code to WebAssembly. It provides a complete game engine with configurable probabilities, ball mechanics, and different game states (Normal and Rush modes).

## Features

- 🎰 Complete pachislo game simulation
- 🎯 Configurable win/lose probabilities
- 🎲 Slot reel generation with customizable symbols
- 🔄 Multiple game states: Normal and Rush modes
- 📊 Real-time game state transitions
- 🌐 Full TypeScript support
- ⚡ High performance WebAssembly implementation

## Installation

```bash
npm install pachislo-wasm
```

## Quick Start

```typescript
import { WasmGame, JsInput, JsOutput, Config, BallsConfig, Probability, SlotProbability } from 'pachislo-wasm';

// Configure ball mechanics
const ballsConfig = new BallsConfig();
ballsConfig.init_balls = 100;
ballsConfig.incremental_balls = 15;
ballsConfig.incremental_rush = 5;

// Configure probabilities for different game modes
const normalProb = new SlotProbability();
normalProb.win = 0.1;
normalProb.fake_win = 0.05;
normalProb.fake_lose = 0.05;

const rushProb = new SlotProbability();
rushProb.win = 0.8;
rushProb.fake_win = 0.1;
rushProb.fake_lose = 0.05;

const rushContinueProb = new SlotProbability();
rushContinueProb.win = 0.9;
rushContinueProb.fake_win = 0.05;
rushContinueProb.fake_lose = 0.03;

// Rush continue probability function
const rushContinueFn = (n: number) => Math.max(0.1, 1.0 - (n * 0.1));

const probability = new Probability(normalProb, rushProb, rushContinueProb, rushContinueFn);
const config = new Config(ballsConfig, probability);

// Set up input handler
const input = new JsInput(this, () => {
    // Return array of commands based on user input
    // Available commands: "LaunchBall", "CauseLottery", "StartGame", "FinishGame", "Finish"
    return ["LaunchBall"];
});

// Set up output handlers
const output = new JsOutput(
    this,
    (transition) => {
        // Handle default state transitions
        console.log('State changed:', transition);
    },
    (gameState) => {
        // Handle game finish
        console.log('Game finished:', gameState);
    },
    (result, slot) => {
        // Handle normal mode lottery
        console.log('Normal lottery:', result, slot);
    },
    (result, slot) => {
        // Handle rush mode lottery
        console.log('Rush lottery:', result, slot);
    },
    (result, slot) => {
        // Handle rush continue lottery
        console.log('Rush continue:', result, slot);
    }
);

// Create and run game
const game = new WasmGame(input, output, config);

// Game loop
function gameLoop() {
    const controlFlow = game.run_step();
    if (controlFlow === ControlFlow.Continue) {
        requestAnimationFrame(gameLoop);
    }
}

gameLoop();
```

## API Reference

### Classes

#### `WasmGame`
Main game controller class.

```typescript
constructor(input: JsInput, output: JsOutput, config: Config)
run_step(): ControlFlow  // Returns Continue or Break
```

#### `Config`
Game configuration container.

```typescript
constructor(balls: BallsConfig, probability: Probability)
```

#### `BallsConfig`
Configuration for ball mechanics.

```typescript
init_balls: number         // Initial number of balls
incremental_balls: number  // Balls gained on normal win
incremental_rush: number   // Balls gained on rush win
```

#### `Probability`
Probability configuration for different game modes.

```typescript
constructor(
    normal: SlotProbability,
    rush: SlotProbability, 
    rush_continue: SlotProbability,
    rush_continue_fn: (n: number) => number
)
```

#### `SlotProbability`
Probability settings for slot outcomes.

```typescript
win: number       // Probability of winning (0.0 - 1.0)
fake_win: number  // Probability of fake win animation
fake_lose: number // Probability of fake lose animation
```

#### `JsInput`
Input handler for game commands.

```typescript
constructor(context: any, wait_for_input: () => string[])
```

#### `JsOutput`
Output handler for game events.

```typescript
constructor(
    context: any,
    default: (transition: Transition) => void,
    finish_game: (state: GameState) => void,
    lottery_normal: (result: LotteryResult, slot: number[]) => void,
    lottery_rush: (result: LotteryResult, slot: number[]) => void,
    lottery_rush_continue: (result: LotteryResult, slot: number[]) => void
)
```

### Types

#### `GameState`
```typescript
type GameState = 
    | "Uninitialized"
    | { Normal: { balls: number } }
    | { Rush: { balls: number; rush_balls: number; n: number } }
```

#### `LotteryResult`
```typescript
type LotteryResult = { Win: Win } | { Lose: Lose }
type Win = "Default" | "FakeWin"
type Lose = "Default" | "FakeLose"
```

#### `Transition`
```typescript
interface Transition {
    before: GameState | null
    after: GameState
}
```

### Available Commands

- `"LaunchBall"` - Launch a ball
- `"CauseLottery"` - Trigger lottery
- `"StartGame"` - Start the game
- `"FinishGame"` - Finish current game
- `"Finish"` - End game completely

## Game States

### Normal Mode
Standard gameplay mode where players launch balls and trigger lotteries with lower win probability.

### Rush Mode
High-probability mode activated after certain conditions. Features:
- Higher win rates
- Rush ball counter
- Continuation mechanics

## Building from Source

### Prerequisites
- Rust 1.70+ with `wasm32-unknown-unknown` target
- wasm-pack

### Build Steps

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the package
wasm-pack build --target web --out-dir pkg

# For Node.js target
wasm-pack build --target nodejs --out-dir pkg-node
```

## Testing

This project includes comprehensive test coverage with unit tests, integration tests, and performance benchmarks.

### Test Structure

The testing suite is organized into several categories:

1. **Unit Tests** - Located in `src/lib.rs` and `src/alias.rs`
2. **Integration Tests** - Located in `tests/integration_tests.rs`
3. **Benchmark Tests** - Located in `tests/benchmark_tests.rs`

### Running Tests

#### Standard Unit Tests
```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test tests::test_convert_string_to_command
```

#### WebAssembly Tests
For testing WebAssembly functionality in a browser environment:

```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Run tests in Chrome/Chromium
wasm-pack test --chrome

# Run tests in Firefox
wasm-pack test --firefox

# Run tests in Safari
wasm-pack test --safari

# Run tests headlessly
wasm-pack test --chrome --headless
```

#### Integration Tests
```bash
# Run integration tests specifically
cargo test --test integration_tests

# Run integration tests in browser
wasm-pack test --chrome --test integration_tests
```

#### Performance Benchmarks
```bash
# Run benchmark tests
cargo test --test benchmark_tests

# Run benchmarks in browser with performance logging
wasm-pack test --chrome --test benchmark_tests
```

### Test Coverage

The test suite covers:

- **Command Processing**: Validation of all game commands
- **Game State Transitions**: Normal, Rush, and Uninitialized states
- **Configuration Management**: Ball settings and probability configurations
- **Type Conversions**: Rust ↔ JavaScript type mappings
- **Error Handling**: Invalid commands and edge cases
- **Memory Management**: Object creation and cleanup
- **Performance**: Initialization, command execution, and memory usage benchmarks

### Test Categories

#### Unit Tests (`src/lib.rs`)
- Command string parsing and validation
- Game initialization and configuration
- Control flow management
- Input/Output handler creation
- Mock JavaScript function integration

#### Unit Tests (`src/alias.rs`)
- Type conversion between Rust and JavaScript
- Lottery result processing
- Game state management
- Configuration object creation
- Probability calculation utilities

#### Integration Tests (`tests/integration_tests.rs`)
- Complete game workflow testing
- Multiple game instance management
- Configuration variation testing
- Command sequence validation
- Concurrent operation testing

#### Benchmark Tests (`tests/benchmark_tests.rs`)
- Game initialization performance
- Command execution speed
- Memory allocation efficiency
- Type conversion overhead
- Concurrent access performance

### Performance Expectations

The benchmark tests validate these performance characteristics:

- **Game Initialization**: < 10ms per instance
- **Command Execution**: < 1ms per command
- **Memory Operations**: < 5ms per allocation cycle
- **Type Conversions**: < 10μs per conversion
- **Concurrent Access**: < 1ms per operation

### Writing New Tests

When adding new functionality, follow these testing patterns:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_feature() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = new_feature(input);
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_feature() {
        // WebAssembly-specific test
        let game = create_test_game();
        let result = game.some_wasm_method();
        assert_valid_result(result);
    }
}
```

### Continuous Integration

Tests are designed to run in CI environments:

```yaml
# Example GitHub Actions workflow
- name: Run Rust tests
  run: cargo test --verbose

- name: Run WASM tests
  run: wasm-pack test --chrome --headless
```

## Development

### Running Tests
```bash
# Run all tests (unit + integration)
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run with detailed output
cargo test -- --nocapture

# Run tests in browser
wasm-pack test --chrome --headless
```

### Linting
```bash
cargo clippy
```

## Dependencies

This package depends on:
- [pachislo](https://crates.io/crates/pachislo) - Core pachislo game engine
- [wasm-bindgen](https://crates.io/crates/wasm-bindgen) - WebAssembly bindings
- [serde](https://crates.io/crates/serde) - Serialization framework

## License

This project is licensed under the same terms as the pachislo crate. Please refer to the pachislo documentation for licensing information.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.