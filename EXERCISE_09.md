# Exercise 9: Lifetimes

## Concepts to Learn

### 1. What are Lifetimes?
Lifetimes ensure that references are always valid. They're Rust's way of preventing dangling references at compile time.

**The problem lifetimes solve:**
```rust
// This won't compile!
fn dangling() -> &String {
    let s = String::from("hello");
    &s  // ERROR: s goes out of scope, reference would be invalid
}
```

### 2. Lifetime Syntax

Lifetimes are denoted with `'a`, `'b`, etc. (apostrophe + name):

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**What this means:**
- Both `x` and `y` must live at least as long as lifetime `'a`
- The returned reference will also live for lifetime `'a`
- Rust ensures the return value is valid for as long as both inputs are

### 3. Lifetime Elision Rules

Rust can infer lifetimes in many cases, so you don't always need to write them:

**Rule 1:** Each reference parameter gets its own lifetime
**Rule 2:** If there's exactly one input lifetime, it's assigned to all output lifetimes
**Rule 3:** If there's a `&self` or `&mut self`, its lifetime is assigned to all output lifetimes

**These are the same:**
```rust
fn first_word(s: &str) -> &str  // Elided
fn first_word<'a>(s: &'a str) -> &'a str  // Explicit
```

### 4. Lifetimes in Structs

When structs hold references, you must annotate lifetimes:

```rust
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn get_text(&self) -> &str {  // Lifetime elided (Rule 3)
        self.text
    }
}

let novel = String::from("Call me Ishmael...");
let excerpt = Excerpt { text: &novel[..] };
```

**What this means:**
- `Excerpt` cannot outlive the `text` reference it holds
- The compiler enforces that `novel` must live longer than `excerpt`

### 5. Multiple Lifetimes

Sometimes you need different lifetimes:

```rust
fn first_or_second<'a, 'b>(x: &'a str, y: &'b str, use_first: bool) -> &'a str
where
    'b: 'a,  // 'b outlives 'a
{
    if use_first {
        x
    } else {
        y as &'a str  // Coerce to 'a
    }
}
```

### 6. The `'static` Lifetime

`'static` means the reference lives for the entire program:

```rust
let s: &'static str = "Hello";  // String literals are 'static
```

### 7. Lifetime Bounds

Combine lifetimes with trait bounds:

```rust
fn print_it<'a, T>(x: &'a T)
where
    T: std::fmt::Display + 'a,
{
    println!("{}", x);
}
```

### 8. Common Patterns

**Returning references:**
```rust
// OK: Input and output tied together
fn first<'a>(data: &'a [i32]) -> &'a i32 {
    &data[0]
}

// OK: Returns reference with same lifetime as self
impl<'a> MyStruct<'a> {
    fn get_data(&self) -> &'a str {
        self.data
    }
}
```

### 9. When You Need Explicit Lifetimes

You need to write lifetimes when:
- Returning references that depend on input references
- Structs that hold references
- Multiple input references with different scopes
- The compiler can't figure it out (it will tell you!)

### 10. Best Practices

- Let the compiler infer when possible (use elision)
- Use descriptive lifetime names in complex cases (`'input`, `'output`)
- Prefer owned types (`String`) over references (`&str`) in structs when possible
- Think about "how long does this reference need to be valid?"

---

## Your Task

**Step 1:** Create a new Rust project
```bash
mkdir exercise_09_lifetimes
cd exercise_09_lifetimes
cargo init
```

**Step 2:** Implement the following functions and structs:

### Basic Lifetime Functions

1. **`longest<'a>(x: &'a str, y: &'a str) -> &'a str`**
   - Returns the longer of two string slices
   - Both inputs must have the same lifetime

2. **`first_word<'a>(s: &'a str) -> &'a str`**
   - Returns the first word (before first space)
   - If no space, return the whole string
   - Hint: Use `.find(' ')` and slicing

3. **`get_first<'a, T>(data: &'a [T]) -> Option<&'a T>`**
   - Returns a reference to the first element
   - Returns `None` if empty
   - This is generic over `T`

### Structs with Lifetimes

4. **`ImportantExcerpt<'a>` struct**
   - Has one field: `part: &'a str`
   - Implement a method `level(&self) -> i32` that returns 3
   - Implement a method `announce_and_return_part(&self, announcement: &str) -> &str`
     - Prints the announcement
     - Returns `self.part`
     - Note: Two input lifetimes, but return uses `self`'s lifetime (Rule 3)

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}
```

### Multiple References

5. **`contains_word<'a>(text: &'a str, word: &str) -> Option<&'a str>`**
   - If `text` contains `word`, return `Some(text)`
   - Otherwise return `None`
   - Note: Return lifetime matches `text`, not `word`

6. **`longest_in_list<'a>(strings: &'a [&str]) -> Option<&'a str>`**
   - Returns the longest string from the list
   - Returns `None` if empty
   - The return lifetime matches the slice lifetime

### Practical Examples

7. **`Parser<'a>` struct**
   - Has field: `content: &'a str`
   - Implement `new(content: &'a str) -> Parser<'a>` constructor
   - Implement `parse_word(&mut self) -> Option<&'a str>` that:
     - Returns the next word from content
     - Updates content to remove that word
     - Returns `None` when no words left
     - Hint: Use `.split_once(' ')` or `.split_whitespace()`

```rust
struct Parser<'a> {
    content: &'a str,
}

impl<'a> Parser<'a> {
    fn new(content: &'a str) -> Self {
        Parser { content }
    }

    fn parse_word(&mut self) -> Option<&'a str> {
        let content = self.content.trim();
        if content.is_empty() {
            return None;
        }

        match content.split_once(' ') {
            Some((word, rest)) => {
                self.content = rest;
                Some(word)
            }
            None => {
                let word = self.content;
                self.content = "";
                Some(word)
            }
        }
    }
}
```

**Step 3:** Create a `main` function demonstrating all functions and structs

**Step 4:** Add this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("hello", "world!"), "world!");
        assert_eq!(longest("rust", "go"), "rust");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word("foo bar baz"), "foo");
    }

    #[test]
    fn test_get_first() {
        let numbers = [1, 2, 3, 4];
        assert_eq!(get_first(&numbers), Some(&1));

        let empty: [i32; 0] = [];
        assert_eq!(get_first(&empty), None);
    }

    #[test]
    fn test_important_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let excerpt = ImportantExcerpt {
            part: &novel[0..22],
        };

        assert_eq!(excerpt.level(), 3);
        assert_eq!(excerpt.announce_and_return_part("Listen!"), "Call me Ishmael. Some");
    }

    #[test]
    fn test_contains_word() {
        let text = "hello world rust";
        assert_eq!(contains_word(text, "world"), Some(text));
        assert_eq!(contains_word(text, "python"), None);
    }

    #[test]
    fn test_longest_in_list() {
        let words = vec!["hi", "hello", "hey"];
        assert_eq!(longest_in_list(&words), Some("hello"));

        let empty: Vec<&str> = vec![];
        assert_eq!(longest_in_list(&empty), None);
    }

    #[test]
    fn test_parser() {
        let text = "hello world rust";
        let mut parser = Parser::new(text);

        assert_eq!(parser.parse_word(), Some("hello"));
        assert_eq!(parser.parse_word(), Some("world"));
        assert_eq!(parser.parse_word(), Some("rust"));
        assert_eq!(parser.parse_word(), None);
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

- Start with the function signature, let the compiler tell you what lifetimes you need
- If you get lifetime errors, read them carefully - they're very helpful
- Use the same lifetime `'a` when the output depends on multiple inputs
- For structs with references, add `<'a>` after the struct name
- In `impl` blocks for lifetime structs: `impl<'a> StructName<'a>`
- Lifetime parameters go before type parameters: `<'a, T>`
- `.trim()` removes whitespace from beginning and end
- `.split_once(' ')` splits on first space, returns `Option<(&str, &str)>`

---

## Challenge (Optional)

- Create a `Book<'a>` struct with `title: &'a str` and `author: &'a str`
- Implement a `summary` method that returns a reference to one of the fields
- Create a function with two different lifetime parameters
- Implement a struct that holds multiple references with different lifetimes

When you're done or need help, let me know and I'll review your code!
