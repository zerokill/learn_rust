# Exercise 8: Traits

## Concepts to Learn

### 1. What are Traits?
Traits are like interfaces in other languages. They define shared behavior that types can implement.

**Defining a trait:**
```rust
trait Describe {
    fn describe(&self) -> String;
}
```

**Implementing a trait:**
```rust
struct Person {
    name: String,
    age: u32,
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}
```

### 2. Trait Methods

**Required methods** (must be implemented):
```rust
trait Animal {
    fn make_sound(&self) -> String;
    fn name(&self) -> &str;
}
```

**Default implementations** (can be overridden):
```rust
trait Animal {
    fn make_sound(&self) -> String;

    fn describe(&self) -> String {
        format!("This animal says: {}", self.make_sound())
    }
}
```

### 3. Common Standard Library Traits

**Debug** - For debugging output:
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
println!("{:?}", p);  // Point { x: 1, y: 2 }
```

**Clone** - For duplicating values:
```rust
#[derive(Clone)]
struct Person {
    name: String,
}

let p1 = Person { name: String::from("Alice") };
let p2 = p1.clone();  // Creates a copy
```

**PartialEq** - For equality comparison:
```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 1, y: 2 };
assert_eq!(p1, p2);  // true
```

### 4. Trait Bounds

Restrict generic types to those that implement certain traits:

```rust
fn print_describe<T: Describe>(item: T) {
    println!("{}", item.describe());
}

// Multiple traits
fn compare<T: PartialEq + Debug>(a: T, b: T) {
    if a == b {
        println!("{:?} equals {:?}", a, b);
    }
}
```

**Where clause** (cleaner for multiple bounds):
```rust
fn complex<T, U>(a: T, b: U) -> String
where
    T: Describe + Clone,
    U: Describe,
{
    format!("{} and {}", a.describe(), b.describe())
}
```

### 5. Deriving Traits

Many common traits can be automatically derived:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

**Commonly derived traits:**
- `Debug` - Debug formatting (`{:?}`)
- `Clone` - Explicit duplication
- `Copy` - Implicit duplication (for stack types)
- `PartialEq` - Equality comparison (`==`)
- `Eq` - Full equality (reflexive)
- `PartialOrd` - Partial ordering (`<`, `>`)
- `Ord` - Total ordering

### 6. Trait Objects (Dynamic Dispatch)

Use traits for polymorphism:

```rust
fn describe_all(items: Vec<&dyn Describe>) {
    for item in items {
        println!("{}", item.describe());
    }
}
```

### 7. Best Practices

- Use traits to define shared behavior across types
- Derive common traits when possible
- Use trait bounds to make generic functions more flexible
- Implement standard library traits (Debug, Clone, etc.) for your types
- Use `&dyn Trait` for trait objects when you need runtime polymorphism

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_08_traits
cd exercise_08_traits
cargo init
```

**Step 2:** Define and implement the following:

### Define a `Shape` Trait

```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Default implementation
    fn describe(&self) -> String {
        format!("Area: {:.2}, Perimeter: {:.2}", self.area(), self.perimeter())
    }
}
```

### Implement for Shapes

1. **Circle struct** with `radius: f64`
   - Implement `Shape` trait
   - Area: π × r²
   - Perimeter: 2 × π × r

2. **Rectangle struct** with `width: f64, height: f64`
   - Implement `Shape` trait
   - Area: width × height
   - Perimeter: 2 × (width + height)

3. **Triangle struct** with `a: f64, b: f64, c: f64` (three sides)
   - Implement `Shape` trait
   - Use Heron's formula for area
   - Perimeter: a + b + c

### Define a `Comparable` Trait

```rust
trait Comparable {
    fn is_larger(&self, other: &Self) -> bool;
    fn is_smaller(&self, other: &Self) -> bool {
        !self.is_larger(other)
    }
}
```

Implement `Comparable` for all three shapes (compare by area).

### Derive Standard Traits

For all structs, derive:
- `Debug`
- `Clone`
- `PartialEq`

```rust
#[derive(Debug, Clone, PartialEq)]
struct Circle {
    radius: f64,
}
```

### Generic Functions

1. **`print_shape_info<T: Shape>(shape: &T)`**
   - Prints the shape's description using the trait method

2. **`largest_shape<T: Shape + Clone>(shapes: &[T]) -> Option<T>`**
   - Returns the shape with the largest area
   - Returns `None` if slice is empty
   - Requires `Clone` to return a copy

3. **`total_area<T: Shape>(shapes: &[T]) -> f64`**
   - Returns the sum of all shapes' areas

**Step 3:** Create a `main` function that:
- Creates instances of each shape
- Demonstrates all trait methods
- Uses the generic functions
- Shows cloning and equality comparison

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle { radius: 5.0 };
        assert!((circle.area() - 78.54).abs() < 0.01);
        assert!((circle.perimeter() - 31.42).abs() < 0.01);
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle { width: 4.0, height: 5.0 };
        assert_eq!(rect.area(), 20.0);
        assert_eq!(rect.perimeter(), 18.0);
    }

    #[test]
    fn test_triangle() {
        let tri = Triangle { a: 3.0, b: 4.0, c: 5.0 };
        assert!((tri.area() - 6.0).abs() < 0.01);
        assert_eq!(tri.perimeter(), 12.0);
    }

    #[test]
    fn test_comparable() {
        let circle = Circle { radius: 5.0 };
        let rect = Rectangle { width: 4.0, height: 5.0 };
        assert!(circle.is_larger(&rect));  // ~78.54 > 20
    }

    #[test]
    fn test_clone() {
        let circle1 = Circle { radius: 5.0 };
        let circle2 = circle1.clone();
        assert_eq!(circle1, circle2);
    }

    #[test]
    fn test_largest_shape() {
        let shapes = vec![
            Circle { radius: 2.0 },
            Circle { radius: 5.0 },
            Circle { radius: 3.0 },
        ];
        let largest = largest_shape(&shapes).unwrap();
        assert_eq!(largest.radius, 5.0);
    }

    #[test]
    fn test_total_area() {
        let shapes = vec![
            Rectangle { width: 2.0, height: 3.0 },  // 6
            Rectangle { width: 4.0, height: 5.0 },  // 20
        ];
        assert_eq!(total_area(&shapes), 26.0);
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

- Use `std::f64::consts::PI` for π
- For `largest_shape`, track the maximum area and corresponding shape
- The `#[derive(...)]` must go above the struct definition
- Trait methods can call other trait methods
- `&Self` in trait definitions means "same type as the implementor"
- Use trait bounds in angle brackets: `<T: Trait>`

---

## Challenge (Optional)

- Add a `Display` trait implementation to format shapes nicely
- Create a `Drawable` trait with a `draw()` method
- Implement a function that accepts `&dyn Shape` (trait objects)
- Add a `Scale` trait with a `scale(&mut self, factor: f64)` method

When you're done or need help, let me know and I'll review your code!
