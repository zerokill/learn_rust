# Exercise 6: Collections - Vec, HashMap, and HashSet

## Concepts to Learn

### 1. Vec<T> - Dynamic Arrays
Vectors are growable arrays stored on the heap. They're one of the most commonly used collections.

**Creating vectors:**
```rust
let mut v1: Vec<i32> = Vec::new();  // Empty vector
let mut v2 = vec![1, 2, 3];          // Using the vec! macro

// Adding elements
v1.push(5);
v1.push(6);
v1.push(7);

// Accessing elements
let third = &v2[2];           // Will panic if index is out of bounds
let third = v2.get(2);        // Returns Option<&T>, safer

// Removing elements
let last = v1.pop();          // Returns Option<T>
```

**Iterating:**
```rust
let v = vec![1, 2, 3];

// Immutable iteration
for num in &v {
    println!("{}", num);
}

// Mutable iteration
let mut v = vec![1, 2, 3];
for num in &mut v {
    *num += 10;
}
```

**Common methods:**
```rust
v.len()              // Number of elements
v.is_empty()         // Check if empty
v.contains(&value)   // Check if contains value
v.clear()            // Remove all elements
```

### 2. HashMap<K, V> - Key-Value Storage
HashMaps store key-value pairs with fast lookup.

**Creating and using:**
```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();

// Inserting
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Accessing
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Returns Option<&V>

match score {
    Some(s) => println!("Score: {}", s),
    None => println!("Team not found"),
}

// Or with unwrap_or
let score = scores.get(&team_name).unwrap_or(&0);

// Updating
scores.insert(String::from("Blue"), 25);  // Overwrites

// Only insert if key doesn't exist
scores.entry(String::from("Yellow")).or_insert(50);

// Update based on old value
let count = scores.entry(String::from("Blue")).or_insert(0);
*count += 10;
```

**Iterating:**
```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### 3. HashSet<T> - Unique Values
HashSets store unique values with fast lookup.

**Creating and using:**
```rust
use std::collections::HashSet;

let mut visited: HashSet<String> = HashSet::new();

// Inserting
visited.insert(String::from("page1"));
visited.insert(String::from("page2"));
visited.insert(String::from("page1"));  // Duplicate, won't be added

// Checking
if visited.contains("page1") {
    println!("Already visited");
}

// Set operations
let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
let set2: HashSet<i32> = [2, 3, 4].iter().cloned().collect();

let union: HashSet<_> = set1.union(&set2).collect();         // {1, 2, 3, 4}
let intersection: HashSet<_> = set1.intersection(&set2).collect();  // {2, 3}
let difference: HashSet<_> = set1.difference(&set2).collect();      // {1}
```

### 4. When to Use Which Collection

- **Vec<T>**: Ordered list, need indexed access, duplicates allowed
- **HashMap<K, V>**: Need to look up values by key
- **HashSet<T>**: Need to track unique values, don't care about order

### 5. Best Practices

- Use `Vec::with_capacity(n)` if you know the size in advance
- Use `.get()` instead of `[]` indexing when index might be invalid
- Remember that HashMap and HashSet don't maintain insertion order
- Use `&` when iterating to avoid moving/consuming the collection
- For HashMaps, keys must implement `Eq` and `Hash` traits

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_06_collections
cd exercise_06_collections
cargo init
```

**Step 2:** Implement the following functions:

### Vector Functions

1. **`sum_vec(numbers: &Vec<i32>) -> i32`**
   - Returns the sum of all numbers in the vector
   - Use a for loop

2. **`double_vec(numbers: &mut Vec<i32>)`**
   - Doubles each element in the vector in place
   - Use a mutable for loop

3. **`find_max(numbers: &Vec<i32>) -> Option<i32>`**
   - Returns the largest number in the vector
   - Returns `None` if the vector is empty
   - Hint: Start with `let mut max = numbers[0]` after checking if empty

### HashMap Functions

4. **`word_count(text: &str) -> HashMap<String, u32>`**
   - Counts how many times each word appears in the text
   - Split text by spaces: `text.split_whitespace()`
   - Use `entry().or_insert(0)` pattern
   - Example: `"hello world hello"` → `{"hello": 2, "world": 1}`

5. **`merge_maps(map1: &HashMap<String, i32>, map2: &HashMap<String, i32>) -> HashMap<String, i32>`**
   - Creates a new HashMap combining both maps
   - If a key exists in both, sum the values
   - Hint: Insert all of map1, then iterate map2 and use `entry().or_insert(0)`

### HashSet Functions

6. **`unique_elements(numbers: &Vec<i32>) -> HashSet<i32>`**
   - Returns a HashSet of unique elements from the vector
   - Iterate and insert each into a HashSet

7. **`has_duplicates(numbers: &Vec<i32>) -> bool`**
   - Returns true if the vector has any duplicate elements
   - Hint: Use a HashSet to track seen elements

8. **`common_elements(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32>`**
   - Returns a vector of elements that appear in both input vectors
   - Convert both to HashSets, use intersection, collect to Vec
   - Hint: You'll need `.cloned()` when collecting

**Step 3:** Create a `main` function that demonstrates all functions with example data

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_sum_vec() {
        assert_eq!(sum_vec(&vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_vec(&vec![]), 0);
        assert_eq!(sum_vec(&vec![-1, 1]), 0);
    }

    #[test]
    fn test_double_vec() {
        let mut v = vec![1, 2, 3];
        double_vec(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&vec![1, 5, 3, 2]), Some(5));
        assert_eq!(find_max(&vec![-10, -5, -20]), Some(-5));
        assert_eq!(find_max(&vec![]), None);
    }

    #[test]
    fn test_word_count() {
        let counts = word_count("hello world hello rust");
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("rust"), Some(&1));
    }

    #[test]
    fn test_merge_maps() {
        let mut map1 = HashMap::new();
        map1.insert(String::from("a"), 1);
        map1.insert(String::from("b"), 2);

        let mut map2 = HashMap::new();
        map2.insert(String::from("b"), 3);
        map2.insert(String::from("c"), 4);

        let merged = merge_maps(&map1, &map2);
        assert_eq!(merged.get("a"), Some(&1));
        assert_eq!(merged.get("b"), Some(&5));  // 2 + 3
        assert_eq!(merged.get("c"), Some(&4));
    }

    #[test]
    fn test_unique_elements() {
        let result = unique_elements(&vec![1, 2, 2, 3, 3, 3]);
        let expected: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_has_duplicates() {
        assert_eq!(has_duplicates(&vec![1, 2, 3, 4]), false);
        assert_eq!(has_duplicates(&vec![1, 2, 2, 3]), true);
        assert_eq!(has_duplicates(&vec![]), false);
    }

    #[test]
    fn test_common_elements() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![3, 4, 5, 6];
        let mut result = common_elements(&v1, &v2);
        result.sort();  // HashSet order is not guaranteed
        assert_eq!(result, vec![3, 4]);
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

- Don't forget `use std::collections::{HashMap, HashSet};` at the top of your file
- For `find_max`, check `if numbers.is_empty()` first
- For `word_count`, use `.to_string()` to convert `&str` to `String`
- For `common_elements`, the pattern is:
  ```rust
  let set1: HashSet<_> = v1.iter().cloned().collect();
  let set2: HashSet<_> = v2.iter().cloned().collect();
  set1.intersection(&set2).cloned().collect()
  ```
- Remember `*` to dereference when modifying values through mutable references
- Use `&` when iterating to avoid consuming the collection

---

## Challenge (Optional)

- Implement `most_frequent_word(text: &str) -> Option<String>` that returns the most common word
- Implement `remove_duplicates(numbers: &mut Vec<i32>)` that removes duplicates in place
- Create a function that groups a vector by some property using HashMap

When you're done or need help, let me know and I'll review your code!
