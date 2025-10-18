# Exercise 7: Error Handling and the `?` Operator

## Concepts to Learn

### 1. Review: `Result<T, E>`
Rust doesn't use exceptions. Instead, functions that can fail return `Result`:

```rust
enum Result<T, E> {
    Ok(T),      // Success with value T
    Err(E),     // Failure with error E
}
```

### 2. Handling Results with `match`

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
```

### 3. The `?` Operator - Propagating Errors

The `?` operator is shorthand for "if this is an error, return it from the current function; otherwise, unwrap the value."

**Without `?`:**
```rust
fn read_and_parse() -> Result<i32, String> {
    let text = match read_file() {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let number = match text.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };

    Ok(number)
}
```

**With `?`:**
```rust
fn read_and_parse() -> Result<i32, String> {
    let text = read_file()?;              // If Err, return early
    let number = text.parse::<i32>()
        .map_err(|e| e.to_string())?;     // Convert error type if needed
    Ok(number)
}
```

**Key points:**
- `?` can only be used in functions that return `Result` or `Option`
- The error types must be compatible
- Makes code much cleaner and easier to read

### 4. `unwrap()` and `expect()`

For when you're **absolutely sure** there won't be an error (or during prototyping):

```rust
let result = divide(10.0, 2.0).unwrap();  // Panics if Err
let result = divide(10.0, 2.0).expect("Division failed");  // Panics with message
```

**When to use:**
- Prototyping/testing
- When you've proven it can't fail (e.g., parsing a hardcoded string)
- **Avoid in production code** - use proper error handling instead

### 5. `unwrap_or()` and `unwrap_or_else()`

Provide default values instead of panicking:

```rust
let result = divide(10.0, 0.0).unwrap_or(0.0);
let result = divide(10.0, 0.0).unwrap_or_else(|_err| {
    println!("Division failed, using default");
    0.0
});
```

### 6. Converting Between Error Types

Use `.map_err()` to convert error types:

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))
}
```

### 7. The `?` Operator with `Option`

`?` also works with `Option`:

```rust
fn get_first_last(list: &[i32]) -> Option<(i32, i32)> {
    let first = list.first()?;  // Returns None if list is empty
    let last = list.last()?;
    Some((*first, *last))
}
```

### 8. Custom Error Types (Preview)

You can create your own error types:

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
```

### 9. Best Practices

- Use `Result` for operations that can fail
- Use `?` for clean error propagation
- Use `unwrap()` only in tests or when you're certain it won't fail
- Provide meaningful error messages
- Use `expect()` with a message instead of `unwrap()` when prototyping
- Prefer `unwrap_or()` or `match` over `unwrap()` in production

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_07_errors
cd exercise_07_errors
cargo init
```

**Step 2:** Implement the following functions:

### Basic Error Handling

1. **`parse_int(s: &str) -> Result<i32, String>`**
   - Parses a string to i32
   - Returns `Ok(number)` on success
   - Returns `Err` with a descriptive message on failure
   - Hint: Use `s.parse::<i32>()` and `.map_err()`

2. **`divide_numbers(a: i32, b: i32) -> Result<i32, String>`**
   - Divides a by b
   - Returns error if b is 0
   - Returns error if division isn't exact (a % b != 0)
   - Example: `divide_numbers(10, 2)` → `Ok(5)`
   - Example: `divide_numbers(10, 3)` → `Err("Not evenly divisible")`

### Using the `?` Operator

3. **`parse_and_double(s: &str) -> Result<i32, String>`**
   - Parses a string to i32 using `parse_int`
   - Doubles the result
   - Use `?` to propagate errors from `parse_int`
   - Example: `parse_and_double("5")` → `Ok(10)`

4. **`calculate(a_str: &str, b_str: &str) -> Result<i32, String>`**
   - Parses both strings to integers using `parse_int`
   - Divides them using `divide_numbers`
   - Use `?` to propagate errors
   - Example: `calculate("10", "2")` → `Ok(5)`
   - Should propagate parsing errors or division errors

### Working with Option

5. **`get_first_element(list: &[i32]) -> Option<i32>`**
   - Returns the first element if it exists
   - Returns `None` if list is empty
   - Hint: Use `.first()` and `.copied()`

6. **`multiply_first_two(list: &[i32]) -> Option<i32>`**
   - Returns the product of the first two elements
   - Returns `None` if list has fewer than 2 elements
   - Use `?` with `get()` to access elements
   - Example: `multiply_first_two(&[3, 4, 5])` → `Some(12)`

### Chaining Operations

7. **`safe_sqrt(x: f64) -> Result<f64, String>`**
   - Returns the square root of x
   - Returns error if x is negative
   - Hint: Use `x.sqrt()` from std library

8. **`parse_and_sqrt(s: &str) -> Result<f64, String>`**
   - Parses string to f64
   - Takes square root using `safe_sqrt`
   - Use `?` to propagate errors from both operations
   - Hint: For parsing f64, use `s.parse::<f64>()`

**Step 3:** Create a `main` function demonstrating all functions with both success and error cases

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("42"), Ok(42));
        assert_eq!(parse_int("-10"), Ok(-10));
        assert!(parse_int("abc").is_err());
        assert!(parse_int("").is_err());
    }

    #[test]
    fn test_divide_numbers() {
        assert_eq!(divide_numbers(10, 2), Ok(5));
        assert_eq!(divide_numbers(20, 4), Ok(5));
        assert!(divide_numbers(10, 0).is_err());
        assert!(divide_numbers(10, 3).is_err());  // Not evenly divisible
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert_eq!(parse_and_double("0"), Ok(0));
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate("10", "2"), Ok(5));
        assert_eq!(calculate("20", "4"), Ok(5));
        assert!(calculate("abc", "2").is_err());  // Parse error
        assert!(calculate("10", "0").is_err());   // Division by zero
        assert!(calculate("10", "3").is_err());   // Not evenly divisible
    }

    #[test]
    fn test_get_first_element() {
        assert_eq!(get_first_element(&[1, 2, 3]), Some(1));
        assert_eq!(get_first_element(&[42]), Some(42));
        assert_eq!(get_first_element(&[]), None);
    }

    #[test]
    fn test_multiply_first_two() {
        assert_eq!(multiply_first_two(&[3, 4, 5]), Some(12));
        assert_eq!(multiply_first_two(&[2, 5]), Some(10));
        assert_eq!(multiply_first_two(&[1]), None);
        assert_eq!(multiply_first_two(&[]), None);
    }

    #[test]
    fn test_safe_sqrt() {
        assert_eq!(safe_sqrt(4.0), Ok(2.0));
        assert_eq!(safe_sqrt(9.0), Ok(3.0));
        assert!(safe_sqrt(-1.0).is_err());
    }

    #[test]
    fn test_parse_and_sqrt() {
        assert_eq!(parse_and_sqrt("4.0"), Ok(2.0));
        assert_eq!(parse_and_sqrt("9.0"), Ok(3.0));
        assert!(parse_and_sqrt("-1.0").is_err());  // Negative number
        assert!(parse_and_sqrt("abc").is_err());   // Parse error
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

- The `?` operator automatically returns early if there's an error
- `.map_err(|e| format!("message: {}", e))` converts error types
- Use `.is_err()` in tests to check if a Result is an error
- For f64 parsing: `s.parse::<f64>()` returns `Result<f64, ParseFloatError>`
- Remember: `?` can only be used in functions returning `Result` or `Option`
- `.copied()` converts `Option<&T>` to `Option<T>` for Copy types

---

## Challenge (Optional)

- Create a custom error enum and use it instead of `String` for error types
- Implement a function that reads from user input and handles all possible errors
- Chain multiple `?` operations in a single function

When you're done or need help, let me know and I'll review your code!
