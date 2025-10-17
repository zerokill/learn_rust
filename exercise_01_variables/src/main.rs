fn main() {
    let age = 37;
    let mut counter: i32 = 0;
    counter +=1;
    let temperature = 23.5;
    let is_learning_rust = true;
    let grade: char = 'A';

    println!("age: {}, counter: {}, temperature: {}, is_learning_rust: {}, grade: {}", age, counter, temperature, is_learning_rust, grade);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_immutable_variable() {
        let x = 42;
        assert_eq!(x, 42);
    }

    #[test]
    fn test_mutable_variable() {
        let mut count = 0;
        count += 1;
        assert_eq!(count, 1);
        count += 1;
        assert_eq!(count, 2);
    }

    #[test]
    fn test_types() {
        let integer: i32 = 100;
        let float: f64 = 3.14;
        let boolean: bool = true;
        let character: char = 'r';

        assert_eq!(integer, 100);
        assert_eq!(float, 3.14);
        assert_eq!(boolean, true);
        assert_eq!(character, 'r');
    }
}

