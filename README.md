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

## Development

### Running Tests
```bash
cargo test
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