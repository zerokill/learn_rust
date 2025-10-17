# Exercise 2: Functions and Control Flow

## Concepts to Learn

### 1. Functions in Rust
Functions are declared with the `fn` keyword:
```rust
fn function_name(parameter: type) -> return_type {
    // function body
    return_value  // implicit return (no semicolon!)
}
```

**Key points:**
- Function names use `snake_case`
- Parameters must have type annotations
- Return type is specified with `->`
- The last expression without a semicolon is the return value (idiomatic Rust!)
- You can use explicit `return` keyword, but it's typically only used for early returns

**Examples:**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // implicit return
}

fn greet(name: &str) {
    println!("Hello, {}!", name);  // no return value (returns unit type ())
}
```

### 2. Control Flow: if/else
```rust
if condition {
    // code
} else if other_condition {
    // code
} else {
    // code
}
```

**Important:**
- No parentheses needed around conditions
- Conditions must be `bool` (no truthy/falsy values like in other languages)
- `if` is an expression, so it can return a value:
```rust
let number = if condition { 5 } else { 6 };
```

### 3. Control Flow: match
`match` is like a powerful switch statement:
```rust
match value {
    1 => println!("One"),
    2 => println!("Two"),
    3 | 4 => println!("Three or Four"),  // multiple patterns
    _ => println!("Something else"),      // catch-all (required!)
}
```

**Key points:**
- `match` must be exhaustive (cover all possibilities)
- Use `_` as a catch-all pattern
- `match` is also an expression and can return values

### 4. Best Practices
- Keep functions small and focused (single responsibility)
- Use descriptive function names that indicate what they do
- Prefer expressions over statements (use implicit returns)
- Use `match` instead of multiple `if/else` when checking specific values

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_02_functions
cd exercise_02_functions
cargo init
```

**Step 2:** Implement the following functions in `src/main.rs`:

1. **`is_even(n: i32) -> bool`**
   - Returns `true` if the number is even, `false` otherwise
   - Hint: Use the modulo operator `%`

2. **`max(a: i32, b: i32) -> i32`**
   - Returns the larger of two numbers
   - Use an `if` expression

3. **`describe_number(n: i32) -> &'static str`**
   - Returns a string description based on the number:
     - If n < 0: return "negative"
     - If n == 0: return "zero"
     - If n > 0 and n <= 10: return "small positive"
     - If n > 10: return "large positive"
   - Use `if/else if/else`

4. **`grade_to_description(grade: char) -> &'static str`**
   - Use a `match` statement to return descriptions:
     - 'A' => "Excellent"
     - 'B' => "Good"
     - 'C' => "Average"
     - 'D' => "Below Average"
     - 'F' => "Failing"
     - _ => "Invalid grade"

**Step 3:** Create a `main` function that:
- Calls each function with test values
- Prints the results using `println!`

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;  // Import functions from parent module

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-2), true);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 10), 10);
        assert_eq!(max(10, 5), 10);
        assert_eq!(max(-3, -8), -3);
        assert_eq!(max(7, 7), 7);
    }

    #[test]
    fn test_describe_number() {
        assert_eq!(describe_number(-5), "negative");
        assert_eq!(describe_number(0), "zero");
        assert_eq!(describe_number(7), "small positive");
        assert_eq!(describe_number(100), "large positive");
    }

    #[test]
    fn test_grade_to_description() {
        assert_eq!(grade_to_description('A'), "Excellent");
        assert_eq!(grade_to_description('B'), "Good");
        assert_eq!(grade_to_description('C'), "Average");
        assert_eq!(grade_to_description('D'), "Below Average");
        assert_eq!(grade_to_description('F'), "Failing");
        assert_eq!(grade_to_description('Z'), "Invalid grade");
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
- Remember: the last expression in a function without a semicolon is the return value
- For `is_even`: a number is even if `n % 2 == 0`
- The `&'static str` type is a string slice with a static lifetime (we'll learn more about lifetimes later - for now, just know it means a string that lives for the entire program)
- Try using `match` as an expression: `match value { ... }` can be assigned to a variable

---

## Challenge (Optional)
If you finish early, try implementing:
- **`fizz_buzz(n: i32) -> String`**: Returns "Fizz" if divisible by 3, "Buzz" if by 5, "FizzBuzz" if by both, or the number as a string otherwise

When you're done or need help, let me know and I'll review your code!
