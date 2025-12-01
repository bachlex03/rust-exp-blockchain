// Integer Overflow Examples
// This file demonstrates how Rust handles integer overflow in different modes
// and the various methods to handle overflow explicitly.

pub fn demonstrate_integer_overflow() {
    println!("=== Integer Overflow in Rust ===\n");

    println!("1. Understanding u8 type (0-255 range)");
    explain_u8_range();

    println!("\n2. Debug Mode: Overflow causes panic");
    demonstrate_debug_overflow();

    println!("\n3. Release Mode: Two's complement wrapping");
    demonstrate_release_wrapping();

    println!("\n4. Explicit Overflow Handling Methods");
    demonstrate_overflow_methods();
}

// u8 can hold values from 0 to 255
fn explain_u8_range() {
    let min: u8 = 0;
    let max: u8 = 255;
    println!("  u8 range: {} to {}", min, max);
    println!("  Maximum value: {}", max);

    // This would overflow if we tried: let overflow = 256u8; // 256u8 is called integer literals
    // But we can't even write that - it's a compile error!
    println!("  Trying to store 256 would overflow!");
}

// In DEBUG mode, Rust checks for overflow and panics
fn demonstrate_debug_overflow() {
    println!("  Running in DEBUG mode (default: cargo run)");
    println!("  Overflow causes program to PANIC (crash with error)");

    let mut value: u8 = 255;
    println!("  Starting value: {}", value);

    // In debug mode, this would panic:
    value = value + 1; // PANIC: attempt to add with overflow

    // To see the panic, uncomment the line above and run: cargo run
    // You'll see: "thread 'main' panicked at 'attempt to add with overflow'"

    println!("  If we tried: value = 255 + 1");
    println!("  Result: PANIC in debug mode!");
}

// In RELEASE mode, Rust uses two's complement wrapping
fn demonstrate_release_wrapping() {
    println!("  Running in RELEASE mode (cargo run --release)");
    println!("  Overflow causes WRAPPING (no panic, but unexpected value)");

    // Two's complement wrapping examples:
    println!("  u8 wrapping examples:");
    println!("    255 + 1 = 0   (wraps to minimum)");
    println!("    255 + 2 = 1");
    println!("    0 - 1 = 255   (wraps to maximum)");

    // Note: We can't demonstrate this easily in code without actually
    // causing overflow, which would panic in debug mode.
    // But in release mode, these would wrap:
    // let x: u8 = 255;
    // let y = x.wrapping_add(1);  // y = 0
}

// Explicit overflow handling methods
fn demonstrate_overflow_methods() {
    println!("\n  Rust provides methods to handle overflow explicitly:");

    // 1. wrapping_* methods - always wrap, never panic
    println!("\n  A. wrapping_* methods (wrap in all modes):");
    let x: u8 = 255;
    let result = x.wrapping_add(1);
    println!("     255.wrapping_add(1) = {}", result);

    let y: u8 = 0;
    let result2 = y.wrapping_sub(1);
    println!("     0.wrapping_sub(1) = {}", result2);

    // 2. checked_* methods - return Option
    println!("\n  B. checked_* methods (return None on overflow):");
    let x: u8 = 255;
    match x.checked_add(1) {
        Some(value) => println!("     255.checked_add(1) = Some({})", value),
        None => println!("     255.checked_add(1) = None (overflow!)"),
    }

    let x: u8 = 200;
    match x.checked_add(50) {
        Some(value) => println!("     200.checked_add(50) = Some({})", value),
        None => println!("     200.checked_add(50) = None (overflow!)"),
    }

    // 3. overflowing_* methods - return (value, overflow_flag)
    println!("\n  C. overflowing_* methods (return value + overflow flag):");
    let x: u8 = 255;
    let (result, overflowed) = x.overflowing_add(1);
    println!(
        "     255.overflowing_add(1) = ({}, overflowed: {})",
        result, overflowed
    );

    let x: u8 = 200;
    let (result, overflowed) = x.overflowing_add(50);
    println!(
        "     200.overflowing_add(50) = ({}, overflowed: {})",
        result, overflowed
    );

    // 4. saturating_* methods - clamp to min/max
    println!("\n  D. saturating_* methods (clamp at boundaries):");
    let x: u8 = 255;
    let result = x.saturating_add(1);
    println!("     255.saturating_add(1) = {} (clamped at max)", result);

    let x: u8 = 200;
    let result = x.saturating_add(100);
    println!("     200.saturating_add(100) = {} (clamped at max)", result);

    let x: u8 = 0;
    let result = x.saturating_sub(1);
    println!("     0.saturating_sub(1) = {} (clamped at min)", result);
}

// Practical example: Safe arithmetic
pub fn safe_arithmetic_example() {
    println!("\n=== Practical Example: Safe Arithmetic ===");

    let user_input: u8 = 250;
    let increment: u8 = 10;

    // Option 1: Check for overflow first
    match user_input.checked_add(increment) {
        Some(result) => {
            println!("Safe addition: {} + {} = {}", user_input, increment, result);
        }
        None => {
            println!("Error: Addition would overflow!");
            // Handle error appropriately (return error, use default, etc.)
        }
    }

    // Option 2: Saturate at boundaries
    let result = user_input.saturating_add(increment);
    println!(
        "Saturating addition: {} + {} = {} (clamped)",
        user_input, increment, result
    );

    // Option 3: Explicitly allow wrapping (when you know it's intentional)
    let result = user_input.wrapping_add(increment);
    println!(
        "Wrapping addition: {} + {} = {} (wrapped)",
        user_input, increment, result
    );
}
