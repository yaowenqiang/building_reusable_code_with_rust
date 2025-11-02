# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust learning project focused on building reusable code with Rust, containing examples and demonstrations of various Rust concepts including generics, iterators, closures, enums, and smart pointers.

## Code Architecture

The project is organized as a collection of standalone Rust source files, each demonstrating specific Rust concepts:

- **Generics**: `generic_demo.rs`, `generic_struct.rs`, `generic_vec.rs` - Shows generic functions, structs, and implementations
- **Iterators**: `iterator_adapters.rs`, `consuming_adaptors.rs`, `IntoIterator.rs`, `fold.rs` - Demonstrates iterator patterns and adaptors
- **Closures**: `closures.rs`, `function_move.rs` - Closure examples and move semantics
- **Control Flow**: `for.rs`, `loop.rs` - Basic iteration constructs
- **Enums**: `enum-option.rs`, `enum_result.rs` - Option and Result type examples
- **Smart Pointers**: `wrappers.rs` - Box, Rc, RefCell, Arc, Mutex, RwLock demonstrations
- **Advanced Patterns**: `chaining.rs`, `lazy_evaluation.rs`, `infinite.rs` - Complex iterator chains and concepts

Each file is a self-contained example with a `main()` function that can be run independently.

## Common Commands

### Running Individual Examples
```bash
# Run any specific example file
rustc [filename].rs && ./[filename]
# OR using cargo run (if Cargo.toml exists)
rustc --edition 2021 [filename].rs -o [filename] && ./[filename]
```

### Compilation
```bash
# Compile a single file
rustc [filename].rs

# Compile with explicit edition
rustc --edition 2021 [filename].rs
```

### Development
```bash
# Check Rust version
rustup show

# Format code
rustfmt [filename].rs

# Check for warnings
rustc --warnings [filename].rs
```

## Key Concepts Demonstrated

- **Generic Programming**: Type-parameterized functions and structs with trait bounds
- **Iterator Pattern**: Lazy evaluation, adapter chains, consuming vs non-consuming adaptors
- **Memory Management**: Stack vs heap allocation, reference counting, interior mutability
- **Error Handling**: Option and Result types for graceful error handling
- **Functional Patterns**: Map, filter, fold operations and closure usage

## File Naming Conventions

- Files use snake_case naming
- Descriptive names indicating the concept being demonstrated
- Examples: `iterator_adapters.rs`, `generic_struct.rs`, `consuming_adaptors.rs`