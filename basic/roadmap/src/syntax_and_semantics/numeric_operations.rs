fn main() {
    println!("=== Numeric Operations in Rust ===\n");

    // Basic arithmetic operations
    demonstrate_basic_operations();

    // Integer division (truncation)
    demonstrate_integer_division();

    // Floating-point operations
    demonstrate_floating_point();

    // Remainder operation
    demonstrate_remainder();

    // Operations with different integer types
    demonstrate_different_types();
}

// Rust supports basic mathematical operations for all number types
fn demonstrate_basic_operations() {
    println!("1. Basic Arithmetic Operations");
    println!("   Rust supports: +, -, *, /, %\n");

    // Addition
    let sum = 5 + 10;
    println!("   Addition: 5 + 10 = {}", sum);

    // Subtraction
    let difference = 95.5 - 4.3;
    println!("   Subtraction: 95.5 - 4.3 = {}", difference);

    // Multiplication
    let product = 4 * 30;
    println!("   Multiplication: 4 * 30 = {}", product);

    // Division
    let quotient = 56.7 / 32.2;
    println!("   Division: 56.7 / 32.2 = {}", quotient);

    // Remainder
    let remainder = 43 % 5;
    println!("   Remainder: 43 % 5 = {}\n", remainder);
}

// Integer division truncates toward zero
fn demonstrate_integer_division() {
    println!("2. Integer Division (Truncation)");
    println!("   Integer division truncates toward zero to the nearest integer\n");

    // Positive numbers
    let quotient1 = 7 / 3;
    println!("   7 / 3 = {} (not 2.333...)", quotient1);
    println!("   → Truncated from 2.333... to 2\n");

    // Negative numbers (also truncates toward zero)
    let quotient2 = -7 / 3;
    println!("   -7 / 3 = {} (not -2.333...)", quotient2);
    println!("   → Truncated from -2.333... to -2 (toward zero)\n");

    // Compare with floating-point division
    let quotient3 = 7.0 / 3.0;
    println!(
        "   7.0 / 3.0 = {} (floating-point, no truncation)",
        quotient3
    );
    println!("   → Floating-point preserves decimal precision\n");
}

// Floating-point operations
fn demonstrate_floating_point() {
    println!("3. Floating-Point Operations");
    println!("   f32 (32-bit) and f64 (64-bit, default)\n");

    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32 explicitly

    println!("   f64 (default): x = {}", x);
    println!("   f32 (explicit): y = {}", y);

    // Operations work the same way
    let sum = x + y as f64; // Need to cast f32 to f64 for operation
    println!("   x + y = {} (after casting)", sum);

    let product = x * 4.5;
    println!("   x * 4.5 = {}\n", product);
}

// Remainder operation
fn demonstrate_remainder() {
    println!("4. Remainder Operation (%)");
    println!("   Returns the remainder after division\n");

    // Basic remainder
    let remainder1 = 43 % 5;
    println!("   43 % 5 = {} (43 = 8*5 + 3)", remainder1);

    let remainder2 = 100 % 7;
    println!("   100 % 7 = {} (100 = 14*7 + 2)", remainder2);

    // Useful for checking divisibility
    let number = 15;
    if number % 3 == 0 {
        println!("   {} is divisible by 3", number);
    }

    // Works with negative numbers too
    let remainder3 = -7 % 3;
    println!(
        "   -7 % 3 = {} (Rust's remainder can be negative)",
        remainder3
    );
    println!("   Note: Rust's % is remainder, not modulo\n");
}

// Operations with different integer types
fn demonstrate_different_types() {
    println!("5. Operations with Different Integer Types");
    println!("   All integer types support the same operations\n");

    // u8 (0-255)
    let a: u8 = 100;
    let b: u8 = 50;
    println!("   u8: {} + {} = {}", a, b, a + b);

    // i32 (signed, default for integers)
    let c: i32 = -50;
    let d: i32 = 25;
    println!("   i32: {} + {} = {}", c, d, c + d);

    // i64
    let e: i64 = 1_000_000;
    let f: i64 = 500_000;
    println!("   i64: {} * {} = {}", e, f, e * f);

    // usize (architecture-dependent, used for indexing)
    let g: usize = 10;
    let h: usize = 20;
    println!("   usize: {} + {} = {}\n", g, h, g + h);

    println!("6. Type Inference in Operations");
    println!("   Rust infers types from the operation context\n");

    // Type is inferred as i32 (default)
    let result1 = 5 + 10;
    println!("   5 + 10 = {} (type: i32, inferred)", result1);

    // Type annotation required when ambiguous
    let result2: u8 = 100 + 50;
    println!("   100 + 50 = {} (type: u8, explicit)", result2);

    // Mixing types requires explicit casting
    let int_val: i32 = 100;
    let float_val: f64 = 3.14;
    // let mix = int_val + float_val; // ERROR: cannot add i32 and f64
    let mix = int_val as f64 + float_val; // OK: explicit cast
    println!(
        "   {} (i32) + {} (f64) = {} (after casting)",
        int_val, float_val, mix
    );
}
