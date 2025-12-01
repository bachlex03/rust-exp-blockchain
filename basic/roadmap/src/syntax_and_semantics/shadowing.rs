fn main() {
    println!("=== Example 1: Shadowing (Current Code) ===");
    example_shadowing();

    println!("\n=== Example 2: Mutability (mut keyword) ===");
    example_mutability();

    println!("\n=== Example 3: Why Shadowing is Safer ===");
    example_shadowing_safety();

    println!("\n=== Example 4: Transformations with Shadowing ===");
    example_shadowing_transformations();
}

// Example 1: Shadowing - creating a NEW variable with the same name
fn example_shadowing() {
    let x = 5; // x is immutable (no mut keyword)

    // This creates a NEW variable named 'x', shadowing the old one
    let x = x + 1; // OK: Using 'let' creates a new variable

    // After shadowing, the new 'x' is also immutable
    println!("After shadowing: x = {}", x);

    // Try to reassign without 'let' - THIS WILL CAUSE A COMPILE ERROR!
    // Uncomment the next line to see the error:
    // x = x + 1;  // ERROR: cannot assign twice to immutable variable `x`
}

// Example 2: Mutability - allows reassignment
fn example_mutability() {
    let mut x = 5; // x is mutable (has mut keyword)

    // Can reassign directly without 'let'
    x = x + 1; // OK: x is mutable, so we can reassign

    println!("After reassignment: x = {}", x);

    // Can reassign again
    x = x * 2; // OK: still mutable
    println!("After another reassignment: x = {}", x);
}

// Example 3: Why shadowing is safer - compile-time protection
fn example_shadowing_safety() {
    let x = 5;

    // Shadowing: must use 'let' keyword
    let x = x + 1; // OK: explicitly creating new variable

    // If you forget 'let', you get a compile error:
    // x = x + 1;  // ERROR: cannot assign to immutable variable

    // This prevents accidental reassignment!
    // With 'mut', you could accidentally do: x = x + 1; and it would work
    // With shadowing, you MUST use 'let', making it clear you're creating a new variable

    println!("x = {} (immutable after shadowing)", x);
}

// Example 4: Transformations with shadowing - common use case
fn example_shadowing_transformations() {
    let spaces = "   "; // String type
    println!("spaces type: string, value: '{}'", spaces);

    // Transform: convert string to number
    // After this, 'spaces' is immutable and is a number, not a string
    let spaces = spaces.len(); // Creates NEW immutable variable (usize type)

    println!("spaces type: usize, value: {}", spaces);

    // The old 'spaces' (string) is gone - we can't accidentally use it
    // The new 'spaces' (number) is immutable - safe from accidental changes

    // Try to change it without 'let' - COMPILE ERROR!
    // spaces = spaces + 1;  // ERROR: cannot assign to immutable variable

    // If we need to change it, we MUST use 'let' (making it explicit)
    let spaces = spaces + 1; // OK: creating new variable
    println!("After transformation: spaces = {}", spaces);
}
