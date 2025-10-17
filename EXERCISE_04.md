# Exercise 4: Structs and Methods

## Concepts to Learn

### 1. Structs - Custom Data Types
Structs let you create custom types by grouping related data together. Think of them like objects in other languages, but more explicit.

**Defining a struct:**
```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

**Creating an instance:**
```rust
let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 25,
    active: true,
};
```

**Accessing fields:**
```rust
println!("Username: {}", user1.username);
```

**Mutable structs:**
```rust
let mut user2 = User {
    username: String::from("bob"),
    email: String::from("bob@example.com"),
    age: 30,
    active: true,
};
user2.age = 31;  // Can modify if the entire struct is mutable
```

### 2. Struct Update Syntax
Create a new instance using fields from another instance:
```rust
let user3 = User {
    email: String::from("charlie@example.com"),
    ..user1  // Use remaining fields from user1
};
```

### 3. Tuple Structs
Structs without named fields:
```rust
struct Point(i32, i32);
struct Color(u8, u8, u8);

let origin = Point(0, 0);
let black = Color(0, 0, 0);
println!("x: {}, y: {}", origin.0, origin.1);
```

### 4. Methods
Methods are functions associated with a struct. They're defined in an `impl` (implementation) block.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (no self) - often used as constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Usage:
let rect = Rectangle { width: 30, height: 50 };
println!("Area: {}", rect.area());  // Method call

let square = Rectangle::square(20);  // Associated function call
```

### 5. `&self`, `&mut self`, and `self`
- **`&self`**: Borrows the struct immutably (most common)
- **`&mut self`**: Borrows the struct mutably (when you need to modify it)
- **`self`**: Takes ownership (rare, used when consuming the struct)

### 6. Best Practices
- Use `&self` for methods that only read data
- Use `&mut self` for methods that modify the struct
- Use associated functions (no `self`) for constructors and factory functions
- Keep structs focused on a single responsibility
- Use descriptive field names

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_04_structs
cd exercise_04_structs
cargo init
```

**Step 2:** Implement a `Rectangle` struct with the following:

**Fields:**
- `width: u32`
- `height: u32`

**Methods in an `impl` block:**

1. **`new(width: u32, height: u32) -> Rectangle`** (associated function)
   - Constructor that creates a new Rectangle
   - Call it like: `Rectangle::new(10, 20)`

2. **`area(&self) -> u32`**
   - Returns the area (width × height)

3. **`perimeter(&self) -> u32`**
   - Returns the perimeter (2 × width + 2 × height)

4. **`is_square(&self) -> bool`**
   - Returns true if width equals height

5. **`can_hold(&self, other: &Rectangle) -> bool`**
   - Returns true if this rectangle can completely hold another rectangle
   - Hint: both width and height must be larger

6. **`scale(&mut self, factor: u32)`**
   - Multiplies both width and height by the factor
   - Modifies the struct in place

**Step 3:** Implement a `Point` tuple struct:
```rust
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point(x, y)
    }

    fn distance_from_origin(&self) -> f64 {
        // Use Pythagorean theorem: sqrt(x² + y²)
        // Hint: use (self.0.pow(2) + self.1.pow(2)) as f64
        // then call .sqrt()
        ((self.0.pow(2) + self.1.pow(2)) as f64).sqrt()
    }
}
```

**Step 4:** Create a `main` function that demonstrates all functionality

**Step 5:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_new() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn test_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_perimeter() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.perimeter(), 60);
    }

    #[test]
    fn test_is_square() {
        let rect = Rectangle::new(10, 20);
        let square = Rectangle::new(10, 10);
        assert_eq!(rect.is_square(), false);
        assert_eq!(square.is_square(), true);
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle::new(10, 20);
        let rect2 = Rectangle::new(5, 10);
        let rect3 = Rectangle::new(15, 25);

        assert_eq!(rect1.can_hold(&rect2), true);
        assert_eq!(rect1.can_hold(&rect3), false);
    }

    #[test]
    fn test_scale() {
        let mut rect = Rectangle::new(10, 20);
        rect.scale(2);
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 40);
    }

    #[test]
    fn test_point() {
        let p = Point::new(3, 4);
        assert_eq!(p.0, 3);
        assert_eq!(p.1, 4);
        assert_eq!(p.distance_from_origin(), 5.0);
    }
}
```

**Step 6:** Run your program and tests
```bash
cargo run
cargo test
```

---

## Tips
- Methods are just functions with `self` as the first parameter
- Use `Rectangle::new()` for associated functions (note the `::`)
- Use `rect.area()` for methods (note the `.`)
- Remember to make the struct mutable (`let mut rect`) when calling `scale()`
- For debugging, you can add `#[derive(Debug)]` above your struct and print with `{:?}`

---

## Challenge (Optional)
- Add a `set_width(&mut self, width: u32)` and `set_height(&mut self, height: u32)` method
- Add a `translate(&mut self, dx: i32, dy: i32)` method to Point that moves it
- Create a `Circle` struct with radius and implement `area()` and `circumference()` methods

When you're done or need help, let me know and I'll review your code!
