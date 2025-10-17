# Exercise 5: Enums and Pattern Matching

## Concepts to Learn

### 1. Enums - Types with Variants
Enums allow you to define a type that can be one of several variants. They're much more powerful in Rust than in most languages.

**Basic enum:**
```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let direction = Direction::North;
```

### 2. Enums with Data
Each variant can hold different types and amounts of data:

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields (like a struct)
    Write(String),              // Single value
    ChangeColor(u8, u8, u8),    // Multiple values (like a tuple)
}

let msg1 = Message::Quit;
let msg2 = Message::Move { x: 10, y: 20 };
let msg3 = Message::Write(String::from("hello"));
let msg4 = Message::ChangeColor(255, 0, 0);
```

### 3. Pattern Matching with `match`
Use `match` to handle different enum variants:

```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Write: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}
```

### 4. The `Option<T>` Enum
Rust doesn't have `null`! Instead, it uses `Option<T>` to represent optional values:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**Usage:**
```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

match find_user(1) {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}
```

**Helper methods:**
```rust
let x: Option<i32> = Some(5);
x.is_some();           // true
x.is_none();           // false
x.unwrap();            // 5 (panics if None!)
x.unwrap_or(0);        // 5, or 0 if None
x.unwrap_or_else(|| 10); // 5, or result of closure if None
```

### 5. The `Result<T, E>` Enum
Used for operations that can succeed or fail:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**Usage:**
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

### 6. Methods on Enums
You can implement methods on enums just like structs:

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Write(text) => println!("{}", text),
            _ => println!("Other message"),
        }
    }
}
```

### 7. `if let` - Simplified Pattern Matching
When you only care about one pattern:

```rust
let some_value = Some(3);

// Instead of:
match some_value {
    Some(3) => println!("three"),
    _ => (),
}

// Use if let:
if let Some(3) = some_value {
    println!("three");
}
```

### 8. Best Practices
- Use enums when a value can be one of a fixed set of variants
- Prefer `Option<T>` over null-like patterns
- Use `Result<T, E>` for operations that can fail
- Use `match` for exhaustive handling, `if let` for single case
- Implement methods on enums when behavior depends on the variant

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_05_enums
cd exercise_05_enums
cargo init
```

**Step 2:** Implement a `TrafficLight` enum:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // Returns how long (in seconds) the light stays on
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 50,
        }
    }
}
```

**Step 3:** Implement a `Shape` enum with variants:

- `Circle(f64)` - holds radius
- `Rectangle(f64, f64)` - holds width and height
- `Triangle(f64, f64, f64)` - holds three sides

Add methods:
```rust
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                // Use Heron's formula: sqrt(s * (s-a) * (s-b) * (s-c))
                // where s = (a + b + c) / 2
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}
```

**Step 4:** Implement helper functions:

1. **`safe_divide(a: f64, b: f64) -> Result<f64, String>`**
   - Returns `Ok(result)` if b is not zero
   - Returns `Err("Division by zero")` if b is zero

2. **`find_even(numbers: &[i32]) -> Option<i32>`**
   - Returns `Some(first_even)` if there's an even number in the slice
   - Returns `None` if no even numbers found
   - Hint: Use a for loop and `if num % 2 == 0`

3. **`get_first_char(s: &str) -> Option<char>`**
   - Returns `Some(first_char)` if string is not empty
   - Returns `None` if string is empty
   - Hint: Use `s.chars().next()`

**Step 5:** Create a `main` function that demonstrates:
- Creating different traffic lights and calling `duration()`
- Creating different shapes and calculating area/perimeter
- Using `safe_divide` with both valid and invalid inputs
- Using `find_even` with different arrays
- Using `get_first_char` with empty and non-empty strings

**Step 6:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 60);
        assert_eq!(TrafficLight::Yellow.duration(), 10);
        assert_eq!(TrafficLight::Green.duration(), 50);
    }

    #[test]
    fn test_shape_circle() {
        let circle = Shape::Circle(5.0);
        assert!((circle.area() - 78.54).abs() < 0.01);
        assert!((circle.perimeter() - 31.42).abs() < 0.01);
    }

    #[test]
    fn test_shape_rectangle() {
        let rect = Shape::Rectangle(4.0, 5.0);
        assert_eq!(rect.area(), 20.0);
        assert_eq!(rect.perimeter(), 18.0);
    }

    #[test]
    fn test_shape_triangle() {
        let triangle = Shape::Triangle(3.0, 4.0, 5.0);
        assert!((triangle.area() - 6.0).abs() < 0.01);
        assert_eq!(triangle.perimeter(), 12.0);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(safe_divide(10.0, 0.0), Err(String::from("Division by zero")));
    }

    #[test]
    fn test_find_even() {
        assert_eq!(find_even(&[1, 3, 4, 7]), Some(4));
        assert_eq!(find_even(&[1, 3, 5]), None);
        assert_eq!(find_even(&[2, 4, 6]), Some(2));
    }

    #[test]
    fn test_get_first_char() {
        assert_eq!(get_first_char("hello"), Some('h'));
        assert_eq!(get_first_char(""), None);
        assert_eq!(get_first_char("x"), Some('x'));
    }
}
```

**Step 7:** Run your program and tests
```bash
cargo run
cargo test
```

---

## Tips
- `match` must be exhaustive - cover all variants
- Use `_` as a catch-all pattern if needed
- For floating point comparisons in tests, check if the difference is small: `(a - b).abs() < 0.01`
- `std::f64::consts::PI` gives you π
- You can destructure enum variants in match arms to access their data
- Use `if let` when you only care about one specific variant

---

## Challenge (Optional)
- Add a `Square(f64)` variant to Shape (side length) and update the methods
- Implement a `Coin` enum with variants for different coin denominations, and a method `value_in_cents(&self) -> u32`
- Create an enum `IpAddr` with variants `V4(u8, u8, u8, u8)` and `V6(String)`

When you're done or need help, let me know and I'll review your code!
