# Exercise 3: Ownership and Borrowing

## Concepts to Learn

### 1. Ownership - Rust's Superpower
Ownership is what makes Rust unique and safe. Every value in Rust has a single **owner**, and when the owner goes out of scope, the value is dropped (memory freed automatically).

**Three Rules of Ownership:**
1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped
3. You can transfer ownership (move) or borrow a reference

**Example:**
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2, s1 is no longer valid!
    // println!("{}", s1);  // ERROR: s1 was moved
    println!("{}", s2);     // OK: s2 owns the string
}
```

### 2. Copy vs Move
- **Copy types** (stored on stack): integers, floats, booleans, chars - these are copied, not moved
- **Move types** (stored on heap): String, Vec, etc. - ownership transfers on assignment

```rust
let x = 5;
let y = x;  // x is copied, both x and y are valid

let s1 = String::from("hello");
let s2 = s1;  // s1 is moved, only s2 is valid
```

### 3. Borrowing - References
Instead of transferring ownership, you can **borrow** a reference:

**Immutable reference** (`&T`):
```rust
fn print_length(s: &String) {  // borrows, doesn't take ownership
    println!("Length: {}", s.len());
}

let my_string = String::from("hello");
print_length(&my_string);  // Pass a reference
println!("{}", my_string);  // Still valid!
```

**Mutable reference** (`&mut T`):
```rust
fn add_world(s: &mut String) {
    s.push_str(" world");
}

let mut my_string = String::from("hello");
add_world(&mut my_string);
println!("{}", my_string);  // "hello world"
```

### 4. Borrowing Rules
**Key rules you must follow:**
1. You can have **either** one mutable reference **OR** any number of immutable references
2. References must always be valid (no dangling references)

```rust
let mut s = String::from("hello");

let r1 = &s;      // OK
let r2 = &s;      // OK - multiple immutable refs
// let r3 = &mut s;  // ERROR: can't have mutable ref while immutable refs exist

println!("{} {}", r1, r2);
// After last use of r1 and r2, we can create a mutable reference
let r3 = &mut s;  // OK now
```

### 5. String vs &str
- `String`: Owned, heap-allocated, growable string
- `&str`: Borrowed string slice, often points to literal text or part of a String

```rust
let owned: String = String::from("hello");
let borrowed: &str = "world";
let slice: &str = &owned[0..3];  // "hel"
```

### 6. Best Practices
- Prefer borrowing over taking ownership when you don't need to own the data
- Use `&str` for function parameters instead of `&String` (more flexible)
- Keep mutable references short-lived
- Let the compiler guide you - ownership errors have great error messages!

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_03_ownership
cd exercise_03_ownership
cargo init
```

**Step 2:** Implement the following functions in `src/main.rs`:

1. **`calculate_length(s: &String) -> usize`**
   - Takes a reference to a String
   - Returns its length
   - The original string should still be usable after calling this function

2. **`make_uppercase(s: &mut String)`**
   - Takes a mutable reference to a String
   - Converts it to uppercase using `s.make_ascii_uppercase()`
   - Modifies the string in place (no return value)

3. **`first_word(s: &str) -> &str`**
   - Takes a string slice
   - Returns a string slice containing just the first word
   - A word is text before the first space
   - Hint: Use `s.find(' ')` which returns `Option<usize>`
   - If no space found, return the whole string
   - Use slicing: `&s[0..index]`

4. **`append_exclamation(s: String) -> String`**
   - Takes ownership of a String
   - Appends "!" to it using `s.push('!')`
   - Returns the modified String
   - The original string cannot be used after calling this (ownership moved)

**Step 3:** Create a `main` function that demonstrates:
- Borrowing (calculate_length)
- Mutable borrowing (make_uppercase)
- String slices (first_word)
- Ownership transfer (append_exclamation)

**Example usage:**
```rust
let s = String::from("hello rust");
println!("Length: {}", calculate_length(&s));
println!("Original still valid: {}", s);

let mut s2 = String::from("hello");
make_uppercase(&mut s2);
println!("Uppercase: {}", s2);

let sentence = "hello world from rust";
println!("First word: {}", first_word(sentence));

let s3 = String::from("hello");
let s4 = append_exclamation(s3);
// println!("{}", s3);  // This would be an error - s3 was moved!
println!("With exclamation: {}", s4);
```

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        // s is still valid here
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_make_uppercase() {
        let mut s = String::from("hello world");
        make_uppercase(&mut s);
        assert_eq!(s, "HELLO WORLD");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word("hello rust programming"), "hello");
    }

    #[test]
    fn test_append_exclamation() {
        let s = String::from("hello");
        let result = append_exclamation(s);
        assert_eq!(result, "hello!");
        // Note: s is no longer valid here (moved)
    }
}
```

**Step 5:** Run your program and tests
```bash
cargo run
cargo test
```

---

## Tips
- If you get ownership errors, read the compiler messages carefully - they're very helpful!
- For `first_word`, use `match` on the `Option` returned by `find`:
  ```rust
  match s.find(' ') {
      Some(index) => &s[0..index],
      None => s,
  }
  ```
- Remember: `&String` borrows, `String` takes ownership
- The difference between `&String` and `&str`: prefer `&str` for parameters (we use `&String` in this exercise to teach the concept)

---

## Challenge (Optional)
- Try implementing `second_word(s: &str) -> Option<&str>` that returns the second word, or `None` if there isn't one
- Experiment: Try to use a variable after it's been moved - see what error message you get!

When you're done or need help, let me know and I'll review your code!
