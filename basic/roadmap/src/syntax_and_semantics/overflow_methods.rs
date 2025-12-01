fn main() {
    println!("=== Integer Overflow in Rust ===\n");

    explain_integer_overflow();
    demonstrate_overflow_methods();
    practical_example();
}

// Understanding u8: can hold values from 0 to 255
fn explain_integer_overflow() {
    println!("1. Understanding u8 Type");
    println!("   u8 can hold values from 0 to 255");
    println!("   Maximum value: 255");
    println!("   If you try to store 256, it OVERFLOWS!\n");

    println!("2. Debug Mode vs Release Mode");
    println!("   DEBUG mode (cargo run):");
    println!("   - Rust checks for overflow");
    println!("   - Program PANICS (crashes) if overflow occurs");
    println!("   - Example: let x: u8 = 255; x = x + 1; → PANIC!\n");

    println!("   RELEASE mode (cargo run --release):");
    println!("   - No overflow checks (for performance)");
    println!("   - Uses TWO'S COMPLEMENT WRAPPING");
    println!("   - 255 + 1 = 0 (wraps to minimum)");
    println!("   - 0 - 1 = 255 (wraps to maximum)");
    println!("   - Program doesn't panic, but value is unexpected!\n");
}

// Demonstrating the different overflow handling methods
fn demonstrate_overflow_methods() {
    println!("3. Explicit Overflow Handling Methods\n");

    let max_u8: u8 = 255;

    // A. wrapping_* methods - always wrap, never panic
    println!("   A. wrapping_* methods (wrap in all modes):");
    println!(
        "      {} + 1 = {} (wrapped)",
        max_u8,
        max_u8.wrapping_add(1)
    );
    println!("      0 - 1 = {} (wrapped)", 0u8.wrapping_sub(1));
    println!("      Use when: You want wrapping behavior in all modes\n");

    // B. checked_* methods - return Option<u8>
    println!("   B. checked_* methods (return None on overflow):");
    match max_u8.checked_add(1) {
        Some(value) => println!("      {} + 1 = Some({})", max_u8, value),
        None => println!("      {} + 1 = None (overflow detected!)", max_u8),
    }
    match 200u8.checked_add(50) {
        Some(value) => println!("      200 + 50 = Some({})", value),
        None => println!("      200 + 50 = None (overflow detected!)"),
    }
    println!("      Use when: You want to detect and handle overflow\n");

    // C. overflowing_* methods - return (value, bool)
    println!("   C. overflowing_* methods (return value + overflow flag):");
    let (result, overflowed) = max_u8.overflowing_add(1);
    println!(
        "      {} + 1 = ({}, overflowed: {})",
        max_u8, result, overflowed
    );

    let (result, overflowed) = 200u8.overflowing_add(50);
    println!("      200 + 50 = ({}, overflowed: {})", result, overflowed);
    println!("      Use when: You need both the result AND overflow status\n");

    // D. saturating_* methods - clamp at boundaries
    println!("   D. saturating_* methods (clamp at min/max):");
    println!(
        "      {} + 1 = {} (clamped at max)",
        max_u8,
        max_u8.saturating_add(1)
    );
    println!(
        "      200 + 100 = {} (clamped at max)",
        200u8.saturating_add(100)
    );
    println!("      0 - 1 = {} (clamped at min)", 0u8.saturating_sub(1));
    println!("      Use when: You want to prevent overflow by clamping\n");
}

// Practical example showing safe arithmetic
fn practical_example() {
    println!("4. Practical Example: Safe Arithmetic\n");

    let user_balance: u8 = 250;
    let deposit: u8 = 10;

    println!(
        "   Scenario: User has {} coins, deposits {}",
        user_balance, deposit
    );

    // Option 1: Check for overflow
    match user_balance.checked_add(deposit) {
        Some(new_balance) => {
            println!("   ✓ Safe: New balance = {}", new_balance);
        }
        None => {
            println!("   ✗ Error: Addition would overflow!");
            println!("   → Handle error: reject transaction or use saturating_add");
        }
    }

    // Option 2: Saturate (clamp at max)
    let safe_balance = user_balance.saturating_add(deposit);
    println!("   → Saturating: Balance clamped at {}", safe_balance);

    // Option 3: Wrap (only if intentional, like in circular buffers)
    let wrapped_balance = user_balance.wrapping_add(deposit);
    println!(
        "   → Wrapping: Balance wrapped to {} (probably wrong!)",
        wrapped_balance
    );

    println!("\n   Best practice: Use checked_* for user input, saturating_* for UI limits");
}
