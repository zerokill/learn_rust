# Exercise 12: Modules and Code Organization

## Concepts to Learn

### 1. Modules - Organizing Code
Modules let you organize code into namespaces for readability and reuse.

**Basic module:**
```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn helper() {  // Private by default
        // ...
    }
}

// Usage
let result = math::add(5, 3);
```

### 2. Privacy Rules

- Everything is **private by default**
- Use `pub` to make items public
- Child modules can access parent's private items
- Parent cannot access child's private items without `pub`

```rust
mod outer {
    pub mod inner {
        pub fn public_fn() {}
        fn private_fn() {}
    }

    fn use_inner() {
        inner::public_fn();  // OK
        // inner::private_fn();  // ERROR
    }
}
```

### 3. File Organization

**Single file (src/main.rs or src/lib.rs):**
```rust
mod utils {
    pub fn helper() {}
}

mod models {
    pub struct User {}
}
```

**Separate files:**
```
src/
  main.rs
  utils.rs      // mod utils { ... }
  models.rs     // mod models { ... }
```

**In main.rs:**
```rust
mod utils;    // Tells Rust to look for utils.rs
mod models;   // Tells Rust to look for models.rs

fn main() {
    utils::helper();
}
```

**Directory modules:**
```
src/
  main.rs
  math/
    mod.rs      // Contains: pub mod operations;
    operations.rs
```

**Or (newer style):**
```
src/
  main.rs
  math.rs       // Contains: pub mod operations;
  math/
    operations.rs
```

### 4. The `use` Keyword

Bring items into scope:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}

// Bring into scope
use math::add;

fn main() {
    let result = add(5, 3);  // Don't need math:: prefix
}
```

**Multiple items:**
```rust
use std::collections::{HashMap, HashSet};

// Glob import (use sparingly)
use std::collections::*;
```

**Renaming:**
```rust
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
```

**Re-exporting:**
```rust
mod inner {
    pub fn function() {}
}

pub use inner::function;  // Makes function available at module root
```

### 5. Paths

**Absolute path** (from crate root):
```rust
crate::math::add(1, 2);
```

**Relative path:**
```rust
// From current module
self::function();

// From parent module
super::function();
```

### 6. `pub use` - Public Re-exports

Make internal modules easier to access:

```rust
mod internal {
    pub mod deep {
        pub fn useful() {}
    }
}

pub use internal::deep::useful;  // Users can now use crate::useful
```

### 7. Struct and Enum Privacy

```rust
mod shapes {
    pub struct Circle {
        pub radius: f64,
        center: (f64, f64),  // Private field
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle {
                radius,
                center: (0.0, 0.0),
            }
        }
    }

    pub enum Color {
        Red,     // Enum variants are public if enum is public
        Blue,
    }
}
```

### 8. Creating a Library Crate

**src/lib.rs:**
```rust
pub mod utils;
pub mod models;

pub fn top_level_function() {}
```

**Usage in tests or binaries:**
```rust
use my_crate::utils;
use my_crate::top_level_function;
```

### 9. Binary vs Library Crates

**Binary crate** (src/main.rs):
- Has a `main` function
- Produces an executable

**Library crate** (src/lib.rs):
- No `main` function
- Meant to be used by other crates

**You can have both!**
```
src/
  main.rs       // Binary crate
  lib.rs        // Library crate
  utils.rs
```

### 10. Best Practices

- Group related functionality into modules
- Use `pub` sparingly - only expose what's needed
- Organize files by feature, not by type
- Use `pub use` to create convenient APIs
- Keep modules focused and cohesive
- Document public APIs with `///` comments

---

## Your Task

**Step 1:** Create a new Rust library project
```bash
mkdir exercise_12_modules
cd exercise_12_modules
cargo init --lib
```

**Step 2:** Create this file structure:

```
src/
  lib.rs
  math.rs
  geometry/
    mod.rs
    shapes.rs
```

### File: src/lib.rs

The main library file that organizes everything:

```rust
pub mod math;
pub mod geometry;

// Re-export commonly used items
pub use geometry::shapes::{Circle, Rectangle};
pub use math::add;
```

### File: src/math.rs

Basic math operations:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Private helper function
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

pub fn simplify_fraction(numerator: i32, denominator: i32) -> (i32, i32) {
    let divisor = gcd(numerator, denominator);
    (numerator / divisor, denominator / divisor)
}
```

### File: src/geometry/mod.rs

Module declaration for geometry:

```rust
pub mod shapes;

// Re-export for convenience
pub use shapes::{Circle, Rectangle};
```

### File: src/geometry/shapes.rs

Shape definitions:

```rust
use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }

    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Result<Self, String> {
        if a + b > c && b + c > a && a + c > b {
            Ok(Triangle { a, b, c })
        } else {
            Err(String::from("Invalid triangle sides"))
        }
    }

    pub fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    pub fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}
```

**Step 3:** Add tests in src/lib.rs:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math;
    use crate::geometry::shapes;

    #[test]
    fn test_math_operations() {
        assert_eq!(math::add(2, 3), 5);
        assert_eq!(math::subtract(5, 3), 2);
        assert_eq!(math::multiply(4, 3), 12);
        assert_eq!(math::divide(10, 2), Ok(5));
        assert!(math::divide(10, 0).is_err());
    }

    #[test]
    fn test_simplify_fraction() {
        assert_eq!(math::simplify_fraction(10, 15), (2, 3));
        assert_eq!(math::simplify_fraction(7, 14), (1, 2));
    }

    #[test]
    fn test_circle() {
        let circle = shapes::Circle::new(5.0);
        assert!((circle.area() - 78.54).abs() < 0.01);
        assert!((circle.perimeter() - 31.42).abs() < 0.01);
    }

    #[test]
    fn test_rectangle() {
        let rect = shapes::Rectangle::new(4.0, 5.0);
        assert_eq!(rect.area(), 20.0);
        assert_eq!(rect.perimeter(), 18.0);
        assert!(!rect.is_square());

        let square = shapes::Rectangle::new(5.0, 5.0);
        assert!(square.is_square());
    }

    #[test]
    fn test_triangle() {
        let tri = shapes::Triangle::new(3.0, 4.0, 5.0).unwrap();
        assert!((tri.area() - 6.0).abs() < 0.01);
        assert_eq!(tri.perimeter(), 12.0);

        assert!(shapes::Triangle::new(1.0, 2.0, 10.0).is_err());
    }

    #[test]
    fn test_reexports() {
        // Test that re-exports work
        let circle = Circle::new(1.0);  // Using re-export from lib.rs
        assert!((circle.area() - PI).abs() < 0.01);

        assert_eq!(add(2, 3), 5);  // Using re-export from lib.rs
    }
}
```

**Step 4:** Create a binary to use your library (src/main.rs):

```rust
use exercise_12_modules::{Circle, Rectangle, add};
use exercise_12_modules::math;
use exercise_12_modules::geometry::shapes::Triangle;

fn main() {
    // Using re-exported items
    println!("2 + 3 = {}", add(2, 3));

    let circle = Circle::new(5.0);
    println!("Circle area: {:.2}", circle.area());

    let rect = Rectangle::new(4.0, 5.0);
    println!("Rectangle area: {:.2}", rect.area());

    // Using full path
    println!("10 * 5 = {}", math::multiply(10, 5));

    // Using constructor that returns Result
    match Triangle::new(3.0, 4.0, 5.0) {
        Ok(tri) => println!("Triangle area: {:.2}", tri.area()),
        Err(e) => println!("Error: {}", e),
    }

    // Demonstrate fraction simplification
    let (num, den) = math::simplify_fraction(10, 15);
    println!("10/15 simplified: {}/{}", num, den);
}
```

**Step 5:** Run tests and binary
```bash
cargo test
cargo run
```

---

## Tips

- `mod module_name;` in one file looks for `module_name.rs` or `module_name/mod.rs`
- In a directory module, create `mod.rs` to declare submodules
- Use `pub mod` to make a module public
- `pub use` re-exports items at a different level
- The crate root is `src/lib.rs` for libraries, `src/main.rs` for binaries
- Tests in `#[cfg(test)]` can access private items in the same file
- Use `super::` to access parent module items
- Use `crate::` for absolute paths from crate root

---

## Challenge (Optional)

- Add a `stats` module with functions like `mean`, `median`, `mode`
- Create nested modules (e.g., `geometry::threed::cube`)
- Add a `constants` module with mathematical constants
- Organize code into multiple files with proper visibility
- Add documentation comments (`///`) to public items

When you're done or need help, let me know and I'll review your code!
