fn main() {
    println!("=== Floating-Point Types in Rust ===\n");

    // 1. Floating-point basics
    demonstrate_float_basics();

    // 2. f32 vs f64
    demonstrate_f32_vs_f64();

    // 3. Floating-point operations
    demonstrate_float_operations();

    // 4. Special values
    demonstrate_special_values();

    // 5. Precision and rounding
    demonstrate_precision();

    // 6. Comparison and ordering
    demonstrate_comparison();

    // 7. Type inference
    demonstrate_float_inference();

    // 8. Practical examples
    demonstrate_practical_examples();
}

// 1. Floating-point basics
fn demonstrate_float_basics() {
    println!("1. Floating-Point Basics");
    println!("   Numbers with decimal points\n");

    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32 with explicit type

    println!("   x = {} (f64 by default)", x);
    println!("   y = {} (f32 explicit)", y);

    // Various decimal numbers
    let a = 3.5;
    let b = 27.0;
    let c = -113.75;
    let d = 0.0078125;

    println!("\n   Various floats:");
    println!("   {}, {}, {}, {}", a, b, c, d);

    println!("\n   Floating-point types:");
    println!("   - f32: 32-bit (single precision)");
    println!("   - f64: 64-bit (double precision, default)\n");
}

// 2. f32 vs f64
fn demonstrate_f32_vs_f64() {
    println!("2. f32 vs f64");
    println!("   Comparing single and double precision\n");

    let f32_val: f32 = 3.14159265358979323846;
    let f64_val: f64 = 3.14159265358979323846;

    println!("   f32: {}", f32_val);
    println!("   f64: {}", f64_val);

    println!("\n   Size in bytes:");
    println!("   f32: {} bytes", std::mem::size_of::<f32>());
    println!("   f64: {} bytes", std::mem::size_of::<f64>());

    println!("\n   Precision:");
    println!("   f32: ~7 decimal digits");
    println!("   f64: ~15 decimal digits");

    println!("\n   f64 is default because:");
    println!("   - Similar speed on modern CPUs");
    println!("   - More precision");
    println!("   - Better for most use cases\n");
}

// 3. Floating-point operations
fn demonstrate_float_operations() {
    println!("3. Floating-Point Operations");
    println!("   Basic arithmetic with floats\n");

    // Addition
    let sum = 5.5 + 10.2;
    println!("   Addition: 5.5 + 10.2 = {}", sum);

    // Subtraction
    let difference = 95.5 - 4.3;
    println!("   Subtraction: 95.5 - 4.3 = {}", difference);

    // Multiplication
    let product = 4.5 * 2.0;
    println!("   Multiplication: 4.5 * 2.0 = {}", product);

    // Division
    let quotient = 56.7 / 32.2;
    println!("   Division: 56.7 / 32.2 = {}", quotient);

    // Remainder (modulo)
    let remainder = 43.5 % 5.0;
    println!("   Remainder: 43.5 % 5.0 = {}", remainder);

    // Common methods
    let num: f64 = -3.7;
    println!("\n   Methods on {}:", num);
    println!("   abs() = {}", num.abs());
    println!("   floor() = {}", num.floor());
    println!("   ceil() = {}", num.ceil());
    println!("   round() = {}", num.round());
    println!("   trunc() = {}", num.trunc());

    println!();
}

// 4. Special values
fn demonstrate_special_values() {
    println!("4. Special Values");
    println!("   Infinity, NaN, and special cases\n");

    // Infinity
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    println!("   Infinity: {}", inf);
    println!("   Negative Infinity: {}", neg_inf);

    // NaN (Not a Number)
    let nan = f64::NAN;
    println!("   NaN: {}", nan);

    // Operations that produce special values
    let div_by_zero = 1.0 / 0.0;
    let neg_div_by_zero = -1.0 / 0.0;
    let sqrt_negative = (-1.0_f64).sqrt();

    println!("\n   Special value operations:");
    println!("   1.0 / 0.0 = {}", div_by_zero);
    println!("   -1.0 / 0.0 = {}", neg_div_by_zero);
    println!("   (-1.0).sqrt() = {}", sqrt_negative);

    // Checking for special values
    println!("\n   Checking special values:");
    println!("   inf.is_infinite() = {}", inf.is_infinite());
    println!("   inf.is_finite() = {}", inf.is_finite());
    println!("   nan.is_nan() = {}", nan.is_nan());
    println!("   (3.14).is_normal() = {}", (3.14_f64).is_normal());

    println!();
}

// 5. Precision and rounding
fn demonstrate_precision() {
    println!("5. Precision and Rounding");
    println!("   Floating-point precision limitations\n");

    // Precision issues
    let result = 1.0 / 5.0;
    println!("   1.0 / 5.0 = {}", result);
    println!("   (not exactly 0.2 due to binary representation)");

    // Demonstrating precision loss
    let a: f64 = 0.1 + 0.2;
    println!("\n   0.1 + 0.2 = {}", a);
    println!("   Expected: 0.3");
    println!("   Actual: {} (precision loss)", a);

    // Comparing floats (don't use ==)
    println!("\n   Comparing floats:");
    println!("   0.1 + 0.2 == 0.3: {}", a == 0.3);
    println!("   Use epsilon comparison instead:");
    let epsilon: f64 = 1e-10;
    println!("   (a - 0.3).abs() < epsilon: {}", (a - 0.3).abs() < epsilon);

    // Rounding
    let num: f64 = 3.14159;
    println!("\n   Rounding {}:", num);
    println!("   round() = {}", num.round());
    println!("   floor() = {}", num.floor());
    println!("   ceil() = {}", num.ceil());
    println!("   trunc() = {}", num.trunc());

    println!();
}

// 6. Comparison and ordering
fn demonstrate_comparison() {
    println!("6. Comparison and Ordering");
    println!("   Comparing floating-point numbers\n");

    let a = 3.14;
    let b = 2.71;

    println!("   a = {}, b = {}", a, b);
    println!("   a > b: {}", a > b);
    println!("   a < b: {}", a < b);
    println!("   a >= b: {}", a >= b);
    println!("   a <= b: {}", a <= b);

    // NaN comparisons
    let nan = f64::NAN;
    println!("\n   NaN comparisons (always false):");
    println!("   NaN == NaN: {}", nan == nan);
    println!("   NaN < 1.0: {}", nan < 1.0);
    println!("   NaN > 1.0: {}", nan > 1.0);

    // total_cmp for sorting
    println!("\n   Using total_cmp for sorting:");
    let mut numbers = vec![3.14, f64::NAN, -1.5, f64::INFINITY, 0.0, -0.0];
    numbers.sort_by(|a, b| a.total_cmp(b));
    println!("   Sorted: {:?}", numbers);

    println!();
}

// 7. Type inference
fn demonstrate_float_inference() {
    println!("7. Type Inference");
    println!("   Rust infers f64 by default for floats\n");

    // Default inference
    let inferred = 3.14;
    println!("   let inferred = 3.14;");
    println!("   Type inferred as f64: {}", inferred);

    // Explicit type
    let explicit: f32 = 3.14;
    println!("\n   let explicit: f32 = 3.14;");
    println!("   Explicit f32: {}", explicit);

    // Type suffix
    let with_suffix = 3.14f32;
    println!("\n   let with_suffix = 3.14f32;");
    println!("   With type suffix: {}", with_suffix);

    // Inference from context
    let mut number = 1.0;
    number = number + 2.5;
    println!("\n   Type inferred from operations: {}", number);

    println!();
}

// 8. Practical examples
fn demonstrate_practical_examples() {
    println!("8. Practical Examples\n");

    // Example 1: Calculate circle area
    println!("   Example 1: Circle area");
    let radius = 5.0;
    let area = std::f64::consts::PI * radius * radius;
    println!("   Radius: {}, Area: {:.2}", radius, area);

    // Example 2: Temperature conversion
    println!("\n   Example 2: Temperature conversion");
    let celsius = 25.0;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("   {}°C = {:.1}°F", celsius, fahrenheit);

    // Example 3: Distance calculation
    println!("\n   Example 3: Distance between points");
    let (x1, y1): (f64, f64) = (0.0, 0.0);
    let (x2, y2): (f64, f64) = (3.0, 4.0);
    let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    println!("   Distance from ({}, {}) to ({}, {}): {}", x1, y1, x2, y2, distance);

    // Example 4: Compound interest
    println!("\n   Example 4: Compound interest");
    let principal: f64 = 1000.0;
    let rate: f64 = 0.05; // 5%
    let years: f64 = 10.0;
    let amount = principal * (1.0 + rate).powf(years);
    println!("   Principal: ${:.2}", principal);
    println!("   Rate: {}%", rate * 100.0);
    println!("   Years: {}", years);
    println!("   Final amount: ${:.2}", amount);

    // Example 5: Safe float comparison
    println!("\n   Example 5: Safe float comparison");
    fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    let x = 0.1 + 0.2;
    let y = 0.3;
    println!("   {} ≈ {}: {}", x, y, approx_equal(x, y, 1e-10));

    println!();
}