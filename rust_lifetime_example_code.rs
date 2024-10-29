//### Ownership and Lifetime Example in Rust
//
//In Rust, **ownership** and **lifetimes** are core features that enforce memory safety without needing a garbage collector. The **ownership** system revolves around three main rules:
//1. Each value in Rust has a single **owner**.
//2. When the owner goes out of scope, the value is dropped.
//3. Ownership can be transferred (moved) or borrowed.
//
//**Lifetimes** ensure that references are valid as long as necessary. Here's a detailed Rust example showcasing ownership, borrowing, and lifetimes:
//
//```rust
fn main() {
    // Ownership: `s1` owns the string "Hello".
    let s1 = String::from("Hello");

    // Move: Ownership of the string is moved to `s2`.
    let s2 = s1;
    // `s1` is no longer valid here.
    // println!("{}", s1); // Uncommenting this would cause a compile-time error.

    // Borrowing: `s2` is borrowed by the function `calculate_length` via a reference.
    let len = calculate_length(&s2);
    println!("The length of '{}' is {}.", s2, len);

    // Mutable Borrowing: Borrowing a mutable reference to change a value.
    let mut s3 = String::from("Goodbye");
    change_string(&mut s3);
    println!("The changed string is '{}'.", s3);

    // Lifetimes: Ensuring the validity of references.
    let r1;
    {
        let s4 = String::from("Rust");
        r1 = longest(&s4, "Language"); // `s4` reference is valid within this scope.
    }
    // `r1` becomes invalid as `s4` is out of scope.
    // println!("Longest string: {}", r1); // Uncommenting this would cause an error.
}

// Function that borrows a string but does not take ownership.
fn calculate_length(s: &String) -> usize {
    s.len() // Just returns the length of the string.
}

// Mutable reference allows modification of the borrowed string.
fn change_string(s: &mut String) {
    s.push_str(", world!"); // Modify the borrowed string.
}

// Function demonstrating lifetimes.
// Both string slices (`&str`) have the same lifetime `'a`.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
//```
//
//### Key Concepts:
//1. **Ownership Transfer**: `s1` is moved to `s2`, and `s1` becomes invalid.
//2. **Borrowing**: `calculate_length` borrows the string (`&s2`) without taking ownership.
//3. **Mutable Borrowing**: `change_string` borrows a mutable reference (`&mut s3`) to modify the string.
//4. **Lifetimes**: `longest` enforces that both string slices live at least as long as the lifetime `'a`, ensuring references don’t become invalid.
//
//---
//
//**Timestamp**: 2024-10-21 10:35:00
//**Summary**: Example of Rust's ownership and lifetime concepts.
//**Lines**: 43
//**Characters**: 1457
//
//```bash
//nvim ownership_lifetime_rust_example.md
//```
