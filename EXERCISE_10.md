# Exercise 10: Iterators and Closures

## Concepts to Learn

### 1. Closures - Anonymous Functions
Closures are anonymous functions that can capture their environment.

**Basic syntax:**
```rust
let add_one = |x| x + 1;
let result = add_one(5);  // 6

// With type annotations
let add = |x: i32, y: i32| -> i32 { x + y };

// Multiline
let complex = |x| {
    let y = x * 2;
    y + 1
};
```

### 2. Capturing Environment

Closures can capture variables from their scope:

```rust
let multiplier = 10;
let multiply = |x| x * multiplier;  // Captures multiplier
println!("{}", multiply(5));  // 50
```

**Three ways to capture:**
- **By reference** (`&T`) - Borrows immutably
- **By mutable reference** (`&mut T`) - Borrows mutably
- **By value** (`T`) - Takes ownership

```rust
// Immutable borrow
let list = vec![1, 2, 3];
let contains_two = || list.contains(&2);

// Mutable borrow
let mut count = 0;
let mut increment = || count += 1;

// Taking ownership (use `move`)
let data = vec![1, 2, 3];
let consume = move || data;  // Takes ownership of data
```

### 3. The `move` Keyword

Forces closure to take ownership:

```rust
let data = vec![1, 2, 3];
let printer = move || println!("{:?}", data);
// data is moved, can't be used here anymore
```

### 4. Iterator Basics

Iterators are lazy - they do nothing until consumed:

```rust
let v = vec![1, 2, 3];
let iter = v.iter();  // Creates iterator, nothing happens yet

// Consuming the iterator
for val in iter {
    println!("{}", val);
}
```

### 5. Iterator Methods

**Transforming (lazy):**
```rust
let v = vec![1, 2, 3];

// map - transform each element
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// filter - keep elements matching condition
let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).collect();

// filter_map - filter and transform
let results: Vec<i32> = v.iter()
    .filter_map(|x| if *x > 1 { Some(x * 2) } else { None })
    .collect();

// take - first n elements
let first_two: Vec<i32> = v.iter().take(2).cloned().collect();

// skip - skip first n elements
let rest: Vec<i32> = v.iter().skip(1).cloned().collect();

// enumerate - get index and value
for (i, val) in v.iter().enumerate() {
    println!("{}: {}", i, val);
}
```

**Consuming:**
```rust
let v = vec![1, 2, 3, 4, 5];

// collect - gather into collection
let vec: Vec<i32> = v.iter().cloned().collect();

// sum - add all elements
let total: i32 = v.iter().sum();

// count - number of elements
let count = v.iter().count();

// any - check if any element matches
let has_even = v.iter().any(|x| x % 2 == 0);

// all - check if all elements match
let all_positive = v.iter().all(|x| *x > 0);

// find - first element matching condition
let first_even = v.iter().find(|x| *x % 2 == 0);

// fold - accumulate a value
let product = v.iter().fold(1, |acc, x| acc * x);

// max / min
let maximum = v.iter().max();
let minimum = v.iter().min();
```

### 6. Chaining Iterator Methods

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let result: i32 = numbers
    .iter()                    // Create iterator
    .filter(|x| *x % 2 == 0)  // Keep evens: 2, 4, 6
    .map(|x| x * 2)           // Double: 4, 8, 12
    .sum();                    // Sum: 24
```

### 7. Creating Your Own Iterators

Implement the `Iterator` trait:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Usage
let mut counter = Counter::new();
assert_eq!(counter.next(), Some(1));
assert_eq!(counter.next(), Some(2));
```

### 8. Best Practices

- Use iterators instead of manual loops when possible (more functional)
- Chain methods for readable transformations
- Use `.cloned()` or `.copied()` when you need owned values from references
- Iterators are zero-cost abstractions (as fast as manual loops!)
- Use closures for short, inline functions
- Use `move` when closures need to own captured variables

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_10_iterators
cd exercise_10_iterators
cargo init
```

**Step 2:** Implement the following functions:

### Closure Practice

1. **`apply_operation(numbers: &[i32], operation: impl Fn(i32) -> i32) -> Vec<i32>`**
   - Takes a slice and a closure
   - Applies the closure to each element
   - Returns a new vector with results
   - Use `.iter().map().collect()`

2. **`filter_by(numbers: &[i32], predicate: impl Fn(&i32) -> bool) -> Vec<i32>`**
   - Takes a slice and a predicate closure
   - Returns vector of elements that match the predicate
   - Use `.iter().filter().cloned().collect()`

### Iterator Methods

3. **`double_positives(numbers: &[i32]) -> Vec<i32>`**
   - Doubles all positive numbers
   - Filters out non-positive numbers
   - Chain `.filter()` and `.map()`

4. **`sum_of_squares(numbers: &[i32]) -> i32`**
   - Squares each number and sums them
   - Use `.map()` and `.sum()`

5. **`find_largest(numbers: &[i32]) -> Option<i32>`**
   - Finds the largest number
   - Use `.iter().max()` and `.copied()`

6. **`partition_even_odd(numbers: &[i32]) -> (Vec<i32>, Vec<i32>)`**
   - Separates even and odd numbers
   - Returns tuple of (evens, odds)
   - Use `.iter().partition()` or two filters

### Advanced Iterator Patterns

7. **`group_by_length(words: &[&str]) -> std::collections::HashMap<usize, Vec<&str>>`**
   - Groups words by their length
   - Returns HashMap where key is length, value is vector of words
   - Use `.iter()` and `.fold()` or manual iteration

8. **`running_sum(numbers: &[i32]) -> Vec<i32>`**
   - Creates a cumulative sum
   - Example: `[1, 2, 3]` → `[1, 3, 6]`
   - Use `.scan()` or manual approach with fold

```rust
fn running_sum(numbers: &[i32]) -> Vec<i32> {
    let mut sum = 0;
    numbers.iter().map(|x| {
        sum += x;
        sum
    }).collect()
}
```

### Creating Custom Iterators

9. **`Fibonacci` struct** - Iterator that generates Fibonacci numbers
   - Has fields `curr: u64` and `next: u64`
   - Implement `Iterator` trait
   - `new()` constructor starting with 0, 1
   - Generates infinite Fibonacci sequence

```rust
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}
```

### Combining Closures and Iterators

10. **`custom_sort_by<F>(numbers: &mut [i32], compare: F)`**
    - Sorts slice using custom comparison function
    - Use `.sort_by()` with the closure
    - `F` is a closure that takes two `&i32` and returns `std::cmp::Ordering`

**Step 3:** Create a `main` function demonstrating:
- All iterator functions
- Example closures
- Chaining multiple iterator methods
- The Fibonacci iterator (take first 10 numbers)

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_operation() {
        let numbers = vec![1, 2, 3, 4];
        let doubled = apply_operation(&numbers, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8]);

        let squared = apply_operation(&numbers, |x| x * x);
        assert_eq!(squared, vec![1, 4, 9, 16]);
    }

    #[test]
    fn test_filter_by() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let evens = filter_by(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
    }

    #[test]
    fn test_double_positives() {
        let numbers = vec![-2, -1, 0, 1, 2, 3];
        assert_eq!(double_positives(&numbers), vec![2, 4, 6]);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14);  // 1 + 4 + 9
        assert_eq!(sum_of_squares(&[2, 3]), 13);     // 4 + 9
    }

    #[test]
    fn test_find_largest() {
        assert_eq!(find_largest(&[1, 5, 3, 2]), Some(5));
        assert_eq!(find_largest(&[]), None);
    }

    #[test]
    fn test_partition_even_odd() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds) = partition_even_odd(&numbers);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_group_by_length() {
        let words = vec!["hi", "hello", "hey", "world"];
        let groups = group_by_length(&words);
        assert_eq!(groups.get(&2), Some(&vec!["hi"]));
        assert_eq!(groups.get(&3), Some(&vec!["hey"]));
        assert_eq!(groups.get(&5).unwrap().len(), 2);  // hello and world
    }

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_sum(&[5]), vec![5]);
    }

    #[test]
    fn test_fibonacci() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_custom_sort_by() {
        let mut numbers = vec![3, 1, 4, 1, 5];
        custom_sort_by(&mut numbers, |a, b| a.cmp(b));
        assert_eq!(numbers, vec![1, 1, 3, 4, 5]);

        let mut numbers = vec![3, 1, 4, 1, 5];
        custom_sort_by(&mut numbers, |a, b| b.cmp(a));  // Reverse
        assert_eq!(numbers, vec![5, 4, 3, 1, 1]);
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

- `impl Fn(...)` as parameter type allows any closure
- `.cloned()` or `.copied()` to convert `&T` to `T`
- `.partition()` splits based on a predicate: `let (yes, no) = iter.partition(predicate);`
- For `group_by_length`, use `.fold()` with a HashMap
- `.scan()` is like `.fold()` but emits intermediate values
- `.take(n)` limits an infinite iterator
- Use `std::cmp::Ordering` for comparison results
- Iterator methods are chainable - build pipelines!

---

## Challenge (Optional)

- Implement a `Range` iterator that works like `0..10`
- Create a function that uses `.fold()` to build a string
- Use `.flat_map()` to flatten nested structures
- Implement `.zip()` to combine two iterators

When you're done or need help, let me know and I'll review your code!
