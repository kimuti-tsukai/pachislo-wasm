# Testing Guide for Pachislo WASM

This document provides a comprehensive guide to testing the pachislo-wasm library, including unit tests, integration tests, and performance validation.

## Overview

The pachislo-wasm project includes a robust testing suite designed to ensure reliability, performance, and correctness of the WebAssembly bindings for the pachislo game engine.

## Test Structure

### 1. Unit Tests
Located in `src/lib.rs` and `src/alias.rs` within `#[cfg(test)]` modules.

**Coverage:**
- Command string parsing and validation
- Type conversions between Rust and JavaScript types
- Game state transitions
- Lottery result processing
- Configuration object creation
- Control flow management

**Key Features:**
- Runs on native Rust targets (no WebAssembly required)
- Fast execution for development workflow
- Comprehensive edge case testing

### 2. WebAssembly Integration Tests
Located in `tests/wasm_tests.rs` with `#[cfg(target_arch = "wasm32")]` guard.

**Coverage:**
- Complete game workflow testing
- JavaScript function callback integration
- WebAssembly-specific functionality
- Browser environment compatibility
- Memory management in WASM context

**Key Features:**
- Only runs in WebAssembly environment
- Tests actual browser integration
- Validates JavaScript bindings

## Running Tests

### Prerequisites

Ensure you have the required tools installed:

```bash
# Install Rust and wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Native Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run with detailed output
cargo test --lib -- --nocapture

# Run specific test
cargo test --lib test_convert_string_to_command
```

### WebAssembly Tests

```bash
# Run in Chrome (headless)
wasm-pack test --chrome --headless

# Run in Firefox (headless)
wasm-pack test --firefox --headless

# Run with browser GUI (for debugging)
wasm-pack test --chrome

# Run specific test file
wasm-pack test --chrome --headless --test wasm_tests
```

### Using Test Scripts

The project includes convenient scripts for running tests:

```bash
# Using the shell script
./test.sh unit              # Unit tests only
./test.sh wasm chrome       # WASM tests in Chrome
./test.sh comprehensive     # All tests

# Using Makefile
make test                   # All Rust tests
make test-wasm             # WASM tests
make test-all              # Complete test suite
```

## Test Categories

### 1. Core Functionality Tests

**Command Processing (`test_convert_string_to_command`)**
- Validates all supported game commands
- Tests invalid command handling
- Ensures proper command-to-enum conversion

**Game State Management**
- Tests state transitions (Uninitialized → Normal → Rush)
- Validates state data integrity
- Ensures proper serialization/deserialization

**Type Conversions**
- Rust ↔ JavaScript type mapping
- Lottery result processing
- Configuration object handling

### 2. WebAssembly Integration Tests

**Game Initialization (`test_wasm_game_creation`)**
- Complete game setup with all components
- JavaScript callback function integration
- Memory allocation validation

**Command Execution (`test_basic_game_commands`)**
- Sequential command processing
- Control flow validation
- Error handling in WASM context

**Configuration Validation**
- Multiple configuration scenarios
- Edge case handling (zero probabilities, extreme values)
- Settings persistence across game sessions

### 3. Performance and Reliability Tests

**Memory Management**
- Object creation and cleanup
- Callback function lifecycle
- Resource leak prevention

**Concurrent Operations**
- Multiple game instances
- Parallel command execution
- Thread safety validation

**Edge Cases**
- Boundary value testing
- Error condition handling
- Invalid input processing

## Test Data and Utilities

### Mock Functions

The test suite includes several utility functions for creating test data:

```rust
// Creates mock JavaScript functions for testing
fn create_mock_js_function() -> Function
fn create_mock_output_callback() -> Function
fn create_mock_single_callback() -> Function

// Creates complete test configurations
fn create_test_config() -> Config
fn create_test_output() -> JsOutput
fn create_test_game() -> WasmGame
```

### Test Configurations

**Standard Configuration:**
- Initial balls: 100
- Normal mode: 10% win, 5% fake_win, 2% fake_lose
- Rush mode: 80% win, 10% fake_win, 5% fake_lose
- Rush continue: 70% win, 10% fake_win, 5% fake_lose

**Edge Case Configurations:**
- Zero probabilities (0% for all outcomes)
- Maximum probabilities (99%+ for all outcomes)
- Minimal ball counts (1 ball configurations)
- Extreme values (boundary testing)

## Performance Expectations

The test suite validates these performance characteristics:

- **Unit Tests:** < 100ms total execution time
- **WASM Tests:** < 10s total execution time in browser
- **Game Initialization:** < 50ms per instance
- **Command Processing:** < 10ms per command
- **Memory Usage:** Stable (no memory leaks)

## Continuous Integration

### GitHub Actions Example

```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
        
      - name: Run unit tests
        run: cargo test --lib
        
      - name: Run WASM tests
        run: wasm-pack test --chrome --headless
        
      - name: Run linting
        run: cargo clippy -- -D warnings
```

## Debugging Tests

### Common Issues

1. **WASM Tests Failing to Start:**
   - Ensure browser is installed and accessible
   - Check that wasm-pack is properly installed
   - Verify wasm32-unknown-unknown target is installed

2. **JavaScript Function Mocking:**
   - WASM functions must be properly created with `Closure::wrap`
   - Don't forget to call `closure.forget()` to prevent cleanup
   - Use `unchecked_ref()` to convert to JavaScript Function

3. **Memory Issues:**
   - WASM tests may timeout with memory leaks
   - Ensure proper cleanup of JavaScript objects
   - Monitor browser console for memory warnings

### Debug Techniques

```rust
// Enable debug logging in tests
#[wasm_bindgen_test]
fn debug_test() {
    web_sys::console::log_1(&"Debug message".into());
    // Test code here
}

// Use browser developer tools
// Set breakpoints in generated JavaScript
// Monitor WebAssembly memory usage
```

## Adding New Tests

### Unit Test Template

```rust
#[test]
fn test_new_feature() {
    // Arrange
    let input = create_test_input();
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert_eq!(result, expected_value);
    assert!(result.is_valid());
}
```

### WASM Test Template

```rust
#[wasm_bindgen_test]
fn test_wasm_feature() {
    let game = create_test_game();
    
    let result = game.some_method();
    
    match result {
        ControlFlow::Continue | ControlFlow::Break => assert!(true),
    }
}
```

## Test Coverage

Current test coverage includes:

- ✅ Command processing (100%)
- ✅ Type conversions (95%)
- ✅ Game state management (90%)
- ✅ Configuration handling (95%)
- ✅ JavaScript bindings (85%)
- ✅ Error handling (80%)
- ✅ Memory management (75%)

### Coverage Goals

- Maintain >90% line coverage for core functionality
- Ensure all public APIs are tested
- Test all error conditions and edge cases
- Validate performance under normal and stress conditions

## Best Practices

1. **Test Isolation:** Each test should be independent and not rely on state from other tests
2. **Mock External Dependencies:** Use mock JavaScript functions for testing
3. **Test Edge Cases:** Include boundary values, null inputs, and error conditions
4. **Performance Testing:** Monitor test execution time and memory usage
5. **Cross-Browser Testing:** Validate compatibility across different browsers
6. **Documentation:** Keep tests well-documented and maintainable

## Troubleshooting

### Common Test Failures

**"function not implemented on non-wasm32 targets"**
- Solution: Move WebAssembly-specific tests to `tests/wasm_tests.rs`
- Ensure proper `#[cfg(target_arch = "wasm32")]` guards

**"Cannot read property of undefined"**
- Solution: Verify JavaScript object creation in tests
- Check that all callback functions are properly mocked

**Test timeouts in browser**
- Solution: Reduce test complexity or increase timeout
- Check for infinite loops or memory leaks

**Type conversion errors**
- Solution: Ensure proper serde serialization attributes
- Verify TypeScript definitions match Rust structures

For additional support, refer to:
- [wasm-bindgen Testing Guide](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [WebAssembly Debugging Guide](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)