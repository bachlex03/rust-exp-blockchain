fn main() {
    println!("=== Ownership in Rust ===\n");

    // 1. Variable scope
    demonstrate_variable_scope();

    // 2. The String type and heap allocation
    demonstrate_string_type();

    // 3. Move semantics
    demonstrate_move_semantics();

    // 4. Clone for deep copy
    demonstrate_clone();

    // 5. Copy trait for stack data
    demonstrate_copy_trait();

    // 6. Ownership and functions
    demonstrate_ownership_and_functions();

    // 7. Return values and ownership
    demonstrate_return_values();

    // 8. Scope and assignment
    demonstrate_scope_and_assignment();
}

// 1. Variable scope
fn demonstrate_variable_scope() {
    println!("1. Variable Scope");
    println!("   Variables are valid from declaration until end of scope\n");

    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("   Inside scope: s = {}", s);
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    println!("   Outside scope: s is no longer accessible\n");
}

// 2. The String type and heap allocation
fn demonstrate_string_type() {
    println!("2. The String Type and Heap Allocation");
    println!("   String is allocated on the heap and can grow\n");

    // String literal (immutable, on stack)
    let s1 = "hello";
    println!("   String literal: {}", s1);

    // String type (mutable, on heap)
    let mut s2 = String::from("hello");
    println!("   String before: {}", s2);

    s2.push_str(", world!");
    println!("   String after push_str: {}", s2);

    println!("\n   String literals are immutable and fixed size");
    println!("   String type is mutable and can grow\n");
}

// 3. Move semantics
fn demonstrate_move_semantics() {
    println!("3. Move Semantics");
    println!("   Heap data is moved, not copied\n");

    // Simple types (Copy trait) - copied
    let x = 5;
    let y = x;
    println!("   Integers (Copy trait):");
    println!("   x = {}, y = {}", x, y);
    println!("   Both x and y are valid\n");

    // String (no Copy trait) - moved
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    println!("   Strings (no Copy trait):");
    println!("   s2 = {}", s2);
    // println!("   s1 = {}", s1); // Error! s1 is no longer valid
    println!("   s1 is no longer valid after move\n");

    println!("   Why? To prevent double-free errors!");
    println!("   Only s2 will free the heap memory\n");
}

// 4. Clone for deep copy
fn demonstrate_clone() {
    println!("4. Clone for Deep Copy");
    println!("   Use .clone() to deeply copy heap data\n");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy of heap data

    println!("   s1 = {}", s1);
    println!("   s2 = {}", s2);
    println!("   Both are valid because heap data was copied\n");

    println!("   Note: clone() can be expensive for large data\n");
}

// 5. Copy trait for stack data
fn demonstrate_copy_trait() {
    println!("5. Copy Trait for Stack Data");
    println!("   Types with Copy trait are copied, not moved\n");

    // Types that implement Copy
    let x = 5; // i32
    let y = x;
    println!("   Integers: x = {}, y = {}", x, y);

    let b1 = true; // bool
    let b2 = b1;
    println!("   Booleans: b1 = {}, b2 = {}", b1, b2);

    let c1 = 'a'; // char
    let c2 = c1;
    println!("   Characters: c1 = {}, c2 = {}", c1, c2);

    let t1 = (1, 2); // tuple of Copy types
    let t2 = t1;
    println!("   Tuples: t1 = {:?}, t2 = {:?}", t1, t2);

    println!("\n   Types that implement Copy:");
    println!("   - All integer types (i32, u64, etc.)");
    println!("   - Boolean (bool)");
    println!("   - Floating point (f32, f64)");
    println!("   - Character (char)");
    println!("   - Tuples of Copy types\n");
}

// 6. Ownership and functions
fn demonstrate_ownership_and_functions() {
    println!("6. Ownership and Functions");
    println!("   Passing values to functions moves or copies them\n");

    let s = String::from("hello"); // s comes into scope
    println!("   Before function: s = {}", s);

    takes_ownership(s); // s's value moves into the function
                        // s is no longer valid here
    // println!("   After function: s = {}", s); // Error!

    let x = 5; // x comes into scope
    println!("   Before function: x = {}", x);

    makes_copy(x); // x is copied into the function
    println!("   After function: x = {}", x); // x is still valid

    println!();
}

fn takes_ownership(some_string: String) {
    println!("   Inside takes_ownership: {}", some_string);
} // some_string goes out of scope and drop is called

fn makes_copy(some_integer: i32) {
    println!("   Inside makes_copy: {}", some_integer);
} // some_integer goes out of scope, nothing special happens

// 7. Return values and ownership
fn demonstrate_return_values() {
    println!("7. Return Values and Ownership");
    println!("   Returning values transfers ownership\n");

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("   s1 from gives_ownership: {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    println!("   s2 created: {}", s2);

    let s3 = takes_and_gives_back(s2); // s2 is moved into function, which moves its return value into s3
    println!("   s3 from takes_and_gives_back: {}", s3);
    // println!("   s2 = {}", s2); // Error! s2 was moved

    println!();
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // returned and moves out to calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // returned and moves out to calling function
}

// 8. Scope and assignment
fn demonstrate_scope_and_assignment() {
    println!("8. Scope and Assignment");
    println!("   Assigning new value drops the old value\n");

    let mut s = String::from("hello");
    println!("   Initial value: {}", s);

    s = String::from("ahoy"); // Old "hello" is dropped immediately
    println!("   After reassignment: {}", s);

    println!("\n   The old value's memory is freed immediately\n");

    // Demonstrating with a function that returns ownership
    let s1 = String::from("hello");
    let len = calculate_length_with_tuple(s1);
    println!("   Using tuple to return ownership:");
    println!("   String: {}, Length: {}", len.0, len.1);
    println!();
}

fn calculate_length_with_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return both the String and its length
}