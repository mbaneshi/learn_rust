**Timestamp:** 2024-10-19 13:27:23  
**Summary:** Expanded explanation of Rust's memory safety, including ownership, borrowing, and lifetimes, with a code example demonstrating these concepts.  
**Lines:** 52  
**Characters:** 3,112  

### Memory Safety in Rust

Rust’s memory safety model revolves around **ownership**, **borrowing**, and **lifetimes**, preventing common issues like dangling pointers, double frees, or data races.

#### Ownership Rules:
1. **Each value has a single owner**: Only one variable can own a value at a time.
2. **When the owner goes out of scope, the value is dropped**: Rust automatically deallocates memory when the owner is no longer in use.
3. **Ownership can be transferred (moved) or borrowed**: When transferring ownership, the original owner can no longer access the value. However, you can **borrow** a reference, either **immutable** (`&T`) or **mutable** (`&mut T`), to allow safe sharing of data.

#### Borrowing & Lifetimes
Borrowing ensures data is used safely without allowing multiple mutable references simultaneously. Rust tracks references' **lifetimes** to guarantee that no references outlive the data they point to.

### Code Example

Let’s break down ownership, borrowing, and lifetimes through this code:

```rust
fn main() {
    let s1 = String::from("Hello");  // s1 owns the String
    let s2 = s1;                     // Ownership is moved from s1 to s2
    // println!("{}", s1);           // Error: s1 is no longer valid

    let s3 = String::from("Rust");
    let len = calculate_length(&s3); // Borrowing s3 (immutable reference)
    println!("Length of '{}': {}", s3, len); // s3 is still valid

    let mut s4 = String::from("Mutable");
    change(&mut s4);                 // Borrowing s4 as mutable
    println!("Changed string: {}", s4);
}

// Borrowing a reference (immutable)
fn calculate_length(s: &String) -> usize {
    s.len()  // Safe to use s, it’s an immutable reference
}

// Borrowing a mutable reference
fn change(s: &mut String) {
    s.push_str(" String");  // Mutate the borrowed value
}
```

#### Key Points:
1. **Move semantics**: `let s2 = s1;` moves ownership from `s1` to `s2`, making `s1` invalid.
2. **Immutable borrowing**: The function `calculate_length` takes an immutable reference (`&String`), allowing read-only access to `s3` without transferring ownership. After borrowing, `s3` is still valid.
3. **Mutable borrowing**: The function `change` accepts a mutable reference (`&mut String`), allowing modification of the string `s4`. Only one mutable reference is allowed at a time, preventing data races.

### Lifetimes in Action
Lifetimes come into play when Rust checks that references are valid for the scope they are used. Here's a lifetime annotation example:

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

Here, both `s1` and `s2` must have the same lifetime `'a`, ensuring that the reference returned is valid for as long as both inputs are.

```bash
nvim rust_memory_safety_borrowing_lifetimes.md
```
