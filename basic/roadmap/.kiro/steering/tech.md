# Technology Stack

## Language & Edition
- **Rust 2024 edition**
- No external dependencies (pure Rust standard library)

## Build System
- **Cargo** - Rust's package manager and build tool

## Common Commands

### Build
```bash
cargo build          # Debug build
cargo build --release # Optimized release build
```

### Run
```bash
cargo run            # Build and run main.rs
cargo run --bin <name> # Run specific binary if multiple exist
```

### Check
```bash
cargo check          # Fast compile check without producing binary
cargo clippy         # Linting (if installed)
cargo fmt            # Format code
```

### Clean
```bash
cargo clean          # Remove build artifacts
```

## Project Configuration
- Package name: `roadmap`
- Version: `0.1.0`
- No external crates used - focuses on standard library features
