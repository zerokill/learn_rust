# Exercise 11: Generic Types

## Concepts to Learn

### 1. What are Generics?
Generics allow you to write code that works with multiple types, reducing duplication.

**Without generics:**
```rust
fn largest_i32(list: &[i32]) -> i32 { /* ... */ }
fn largest_char(list: &[char]) -> char { /* ... */ }
```

**With generics:**
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### 2. Generic Functions

```rust
fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

// Multiple type parameters
fn mix<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}
```

### 3. Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

**Multiple type parameters:**
```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair = Pair { first: 5, second: "hello" };
```

### 4. Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 5. Generic Methods

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Method only for specific type
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### 6. Trait Bounds

Constrain generic types to those implementing specific traits:

```rust
fn print_it<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Multiple bounds
fn compare<T: PartialOrd + std::fmt::Debug>(a: T, b: T) {
    if a > b {
        println!("{:?} is larger", a);
    }
}

// Where clause for readability
fn complex<T, U>(a: T, b: U)
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug,
{
    println!("{}", a);
    println!("{:?}", b);
}
```

### 7. Generic with Lifetimes and Traits

```rust
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

### 8. Associated Types

Instead of generic parameters on traits:

```rust
trait Iterator {
    type Item;  // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

### 9. The `Default` Trait

Provide default values for generic types:

```rust
fn create_point<T: Default>() -> Point<T> {
    Point {
        x: T::default(),
        y: T::default(),
    }
}

let p: Point<i32> = create_point();  // Point { x: 0, y: 0 }
```

### 10. Best Practices

- Use generics to reduce code duplication
- Add trait bounds when you need specific functionality
- Use `where` clauses for complex bounds
- Generic code has zero runtime cost (monomorphization)
- Prefer generics over concrete types for libraries
- Use descriptive type parameter names for clarity (not just `T`)

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_11_generics
cd exercise_11_generics
cargo init
```

**Step 2:** Implement the following generic types and functions:

### Generic Functions

1. **`swap<T>(a: T, b: T) -> (T, T)`**
   - Swaps two values and returns them
   - Example: `swap(1, 2)` → `(2, 1)`

2. **`first<T>(items: &[T]) -> Option<&T>`**
   - Returns reference to first element
   - Works with any type

3. **`max<T: PartialOrd>(a: T, b: T) -> T`**
   - Returns the larger of two values
   - Requires `PartialOrd` trait for comparison

### Generic Structs

4. **`Container<T>` struct**
   - Has one field: `value: T`
   - Implement these methods:
     - `new(value: T) -> Self` - constructor
     - `get(&self) -> &T` - returns reference to value
     - `set(&mut self, value: T)` - updates value

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn set(&mut self, value: T) {
        self.value = value;
    }
}
```

5. **`Pair<T, U>` struct**
   - Has fields: `first: T` and `second: U`
   - Implement these methods:
     - `new(first: T, second: U) -> Self`
     - `first(&self) -> &T`
     - `second(&self) -> &U`
     - `swap(self) -> Pair<U, T>` - swaps the pair types

### Generic with Trait Bounds

6. **`print_all<T: std::fmt::Display>(items: &[T])`**
   - Prints each item in the slice
   - Requires `Display` trait

7. **`sum_all<T>(items: &[T]) -> T`**
   - Sums all elements in a slice
   - Requires: `T: std::ops::Add<Output = T> + Default + Copy`
   - Use `T::default()` for initial value

```rust
fn sum_all<T>(items: &[T]) -> T
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    let mut sum = T::default();
    for item in items {
        sum = sum + *item;
    }
    sum
}
```

8. **`find_min<T: PartialOrd + Clone>(items: &[T]) -> Option<T>`**
   - Returns the minimum element
   - Returns `None` if empty
   - Requires `PartialOrd` for comparison and `Clone` to return a copy

### Generic Structs with Multiple Types

9. **`Point<T>` struct** where T can be different for x and y
   - Actually make it `Point<T, U>` with `x: T` and `y: U`
   - Implement method `mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>`
     - Takes self's x and other's y

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

### Generic Wrapper Type

10. **`Wrapper<T>` struct** with transformation methods
    - Has field: `value: T`
    - Implement `map<U, F>(self, f: F) -> Wrapper<U>` where `F: FnOnce(T) -> U`
      - Transforms the wrapped value using function f

```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn map<U, F>(self, f: F) -> Wrapper<U>
    where
        F: FnOnce(T) -> U,
    {
        Wrapper { value: f(self.value) }
    }

    fn unwrap(self) -> T {
        self.value
    }
}
```

**Step 3:** Create a `main` function demonstrating all generic types and functions

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        assert_eq!(swap(1, 2), (2, 1));
        assert_eq!(swap("hello", "world"), ("world", "hello"));
    }

    #[test]
    fn test_first() {
        assert_eq!(first(&[1, 2, 3]), Some(&1));
        assert_eq!(first(&["a", "b"]), Some(&"a"));
        let empty: &[i32] = &[];
        assert_eq!(first(empty), None);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 10), 10);
        assert_eq!(max(3.14, 2.71), 3.14);
        assert_eq!(max('a', 'z'), 'z');
    }

    #[test]
    fn test_container() {
        let mut container = Container::new(42);
        assert_eq!(*container.get(), 42);

        container.set(100);
        assert_eq!(*container.get(), 100);
    }

    #[test]
    fn test_pair() {
        let pair = Pair::new(1, "hello");
        assert_eq!(*pair.first(), 1);
        assert_eq!(*pair.second(), "hello");

        let swapped = pair.swap();
        assert_eq!(*swapped.first(), "hello");
        assert_eq!(*swapped.second(), 1);
    }

    #[test]
    fn test_sum_all() {
        assert_eq!(sum_all(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_all(&[1.5, 2.5, 3.0]), 7.0);
    }

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(&[3, 1, 4, 1, 5]), Some(1));
        assert_eq!(find_min(&[2.5, 1.2, 3.7]), Some(1.2));
        let empty: &[i32] = &[];
        assert_eq!(find_min(empty), None);
    }

    #[test]
    fn test_point_mixup() {
        let p1 = Point::new(5, 10.4);
        let p2 = Point::new("Hello", 'c');

        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, 'c');
    }

    #[test]
    fn test_wrapper_map() {
        let w = Wrapper::new(5);
        let w2 = w.map(|x| x * 2);
        assert_eq!(w2.unwrap(), 10);

        let w3 = Wrapper::new("hello");
        let w4 = w3.map(|s| s.len());
        assert_eq!(w4.unwrap(), 5);
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

- Generic type parameters go in angle brackets: `<T>`
- For impl blocks: `impl<T> StructName<T>`
- Use trait bounds with `:` - `T: Trait`
- Multiple bounds with `+` - `T: Trait1 + Trait2`
- Use `where` clause for complex bounds
- `Copy` types can be copied implicitly, `Clone` needs `.clone()`
- `Default::default()` or `T::default()` gives default value
- `std::ops::Add` is the trait for `+` operator
- Generic parameters are erased at compile time (zero cost!)

---

## Challenge (Optional)

- Implement a generic `Stack<T>` with push/pop
- Create a `Result`-like enum with `Success<T>` and `Failure<E>`
- Implement `From` and `Into` traits for your generic types
- Create a function with three or more generic type parameters

When you're done or need help, let me know and I'll review your code!
