fn main() {
    println!("=== Functions in Rust ===\n");

    // 1. Basic function definition
    demonstrate_basic_functions();

    // 2. Function parameters
    demonstrate_parameters();

    // 3. Multiple parameters
    demonstrate_multiple_parameters();

    // 4. Statements vs Expressions
    demonstrate_statements_vs_expressions();

    // 5. Functions with return values
    demonstrate_return_values();

    // 6. The semicolon trap
    demonstrate_semicolon_trap();
}

// Basic function definition
fn demonstrate_basic_functions() {
    println!("1. Basic Function Definition");
    println!("   Functions use snake_case naming convention\n");

    println!("   Calling another_function:");
    another_function();

    println!("   → Functions can be defined before or after main()");
    println!("   → Rust doesn't care about order, only that they're defined\n");
}

fn another_function() {
    println!("   Another function.");
}

// Function parameters - must specify types!
fn demonstrate_parameters() {
    println!("2. Function Parameters");
    println!("   Parameter types MUST be declared in function signature\n");

    another_function_with_param(5);
    another_function_with_param(42);

    println!("   → Type annotations required: fn func_name(param: type)");
    println!("   → This helps compiler give better error messages\n");
}

fn another_function_with_param(x: i32) {
    println!("   The value of x is: {}", x);
}

// Multiple parameters
fn demonstrate_multiple_parameters() {
    println!("3. Multiple Parameters");
    println!("   Separate parameters with commas\n");

    print_labeled_measurement(5, 'h');
    print_labeled_measurement(100, 'm');

    println!("   → Each parameter needs its type specified\n");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("   The measurement is: {}{}", value, unit_label);
}

// Statements vs Expressions
fn demonstrate_statements_vs_expressions() {
    println!("4. Statements vs Expressions");
    println!("   Important distinction in Rust!\n");

    println!("   Statements: perform action, don't return value");
    println!("   - let x = 5; is a statement");
    println!("   - Function definitions are statements\n");

    println!("   Expressions: evaluate to a value");
    println!("   - 5 + 6 is an expression (evaluates to 11)");
    println!("   - Function calls are expressions");
    println!("   - Blocks {{}} are expressions\n");

    // Block expression example
    let y = {
        let x = 3;
        x + 1 // No semicolon! This is an expression
    };

    println!("   Block expression example:");
    println!("   let y = {{");
    println!("       let x = 3;");
    println!("       x + 1  // No semicolon!");
    println!("   }};");
    println!("   → y = {}\n", y);

    println!("   Key point: Expressions don't end with semicolons!");
    println!("   Adding semicolon turns expression into statement\n");
}

// Functions with return values
fn demonstrate_return_values() {
    println!("5. Functions with Return Values");
    println!("   Return type specified with -> type\n");

    let x = five();
    println!("   let x = five();");
    println!("   → x = {}\n", x);

    let result = plus_one(5);
    println!("   let result = plus_one(5);");
    println!("   → result = {}\n", result);

    let sum = add(10, 20);
    println!("   let sum = add(10, 20);");
    println!("   → sum = {}\n", sum);

    println!("   Return value = last expression in function body");
    println!("   No 'return' keyword needed (but can use it)\n");
}

// Function that returns a value
fn five() -> i32 {
    5 // No semicolon - this is an expression that returns 5
}

// Function with parameter that returns value
fn plus_one(x: i32) -> i32 {
    x + 1 // No semicolon - returns x + 1
}

// Function with multiple parameters returning value
fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon - returns a + b
}

// The semicolon trap - common mistake!
fn demonstrate_semicolon_trap() {
    println!("6. The Semicolon Trap");
    println!("   Adding semicolon to return expression causes error!\n");

    println!("   CORRECT (no semicolon):");
    println!("   fn plus_one(x: i32) -> i32 {{");
    println!("       x + 1  // Expression - returns value");
    println!("   }}\n");

    println!("   WRONG (with semicolon):");
    println!("   fn plus_one(x: i32) -> i32 {{");
    println!("       x + 1;  // Statement - returns ()");
    println!("   }}");
    println!("   → ERROR: expected i32, found ()\n");

    // Demonstrate with explicit return
    let result = explicit_return_example(10);
    println!("   Using explicit 'return' keyword:");
    println!("   fn explicit_return_example(x: i32) -> i32 {{");
    println!("       return x * 2;  // Explicit return");
    println!("   }}");
    println!("   → result = {}\n", result);

    println!("   Note: Most functions use implicit return (no semicolon)");
    println!("   Use 'return' for early returns\n");
}

fn explicit_return_example(x: i32) -> i32 {
    return x * 2; // Explicit return with semicolon is OK
}
