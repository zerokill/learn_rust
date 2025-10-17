fn calculate_length(s: &String) -> usize {
    s.len()
}

fn make_uppercase(s: &mut String) {
    s.make_ascii_uppercase();
}

fn first_word(s: &str) -> &str {
    match s.find(" ") {
        Some(index) => &s[0..index],
        None => s,
    }
}

fn second_word(s: &str) -> Option<&str> {
    s.split(" ").nth(1)
}

fn append_exclamation(mut s: String) -> String {
    s.push('!');
    s
}

fn main() {
    let test_string = String::from("asdasd");
    calculate_length(&test_string);

    let s = String::from("hello rust");
    println!("Length: {}", calculate_length(&s));
    println!("Original still valid: {}", s);

    let mut s2 = String::from("hello");
    make_uppercase(&mut s2);
    println!("Uppercase: {}", s2);

    let sentence = "hello world from rust";
    println!("First word: {}", first_word(sentence));

    let s3 = String::from("hello");
    let s4 = append_exclamation(s3);
    // println!("{}", s3);  // This would be an error - s3 was moved!
    println!("With exclamation: {}", s4);

    if let Some(word) = second_word(sentence) {
        println!("Second word: {}", word);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        // s is still valid here
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_make_uppercase() {
        let mut s = String::from("hello world");
        make_uppercase(&mut s);
        assert_eq!(s, "HELLO WORLD");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word("hello rust programming"), "hello");
    }

    #[test]
    fn test_append_exclamation() {
        let s = String::from("hello");
        let result = append_exclamation(s);
        assert_eq!(result, "hello!");
        // Note: s is no longer valid here (moved)
    }
}
