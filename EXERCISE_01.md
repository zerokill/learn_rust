# Exercise 1: Variables and Basic Types

## Concepts to Learn

### 1. Creating a Rust Project
Rust uses **Cargo** as its build system and package manager. Every Rust project starts with `cargo new` or `cargo init`.

- `cargo new project_name` - Creates a new directory with a Rust project
- `cargo init` - Initializes a Rust project in the current directory
- `cargo build` - Compiles your project
- `cargo run` - Compiles and runs your project
- `cargo test` - Runs your tests

### 2. Variables in Rust
- Variables are **immutable by default** (this is a key Rust best practice!)
- Use `let` to declare a variable: `let x = 5;`
- Use `mut` to make a variable mutable: `let mut y = 10;`
- Rust has **type inference** but you can specify types: `let x: i32 = 5;`

### 3. Basic Types
- **Integers**: `i8`, `i16`, `i32`, `i64`, `i128` (signed) and `u8`, `u16`, `u32`, `u64`, `u128` (unsigned)
- **Floats**: `f32`, `f64`
- **Boolean**: `bool` (true/false)
- **Character**: `char` (single Unicode character, uses single quotes: `'a'`)
- **String slice**: `&str` (we'll learn more about strings later)

### 4. Best Practices
- Prefer immutable variables when possible (helps prevent bugs!)
- Use descriptive variable names with `snake_case`
- Let the compiler infer types when it's obvious, specify when it helps readability

---

## Your Task

**Step 1:** Create a new Rust project for this exercise
```bash
mkdir exercise_01_variables
cd exercise_01_variables
cargo init
```

**Step 2:** Open `src/main.rs` and implement the following:

Create a `main` function that:
1. Declares an immutable integer variable called `age` and sets it to your age
2. Declares a mutable integer variable called `counter` and sets it to 0
3. Increments `counter` by 1 (using `counter = counter + 1;` or `counter += 1;`)
4. Declares a float variable called `temperature` and sets it to 23.5
5. Declares a boolean variable called `is_learning_rust` and sets it to true
6. Declares a char variable called `grade` and sets it to 'A'
7. Prints all these variables using `println!` macro

**Example of println!:**
```rust
println!("Age: {}", age);
println!("Counter: {}, Temperature: {}", counter, temperature);
```

**Step 3:** Add this test module at the bottom of your `src/main.rs` file:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_immutable_variable() {
        let x = 42;
        assert_eq!(x, 42);
    }

    #[test]
    fn test_mutable_variable() {
        let mut count = 0;
        count += 1;
        assert_eq!(count, 1);
        count += 1;
        assert_eq!(count, 2);
    }

    #[test]
    fn test_types() {
        let integer: i32 = 100;
        let float: f64 = 3.14;
        let boolean: bool = true;
        let character: char = 'R';

        assert_eq!(integer, 100);
        assert_eq!(float, 3.14);
        assert_eq!(boolean, true);
        assert_eq!(character, 'R');
    }
}
```

**Step 4:** Run your program and tests
```bash
cargo run      # Should print your variables
cargo test     # Should show all tests passing
```

---

## Tips
- If you get a "unused variable" warning, that's okay for now! Rust warns you about unused code.
- To silence the warning temporarily, prefix the variable with an underscore: `let _x = 5;`
- Try changing `age` after declaring it - Rust will give you a helpful error about immutability!

---

When you're done or need help, let me know and I'll review your code!
