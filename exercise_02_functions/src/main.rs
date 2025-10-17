fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn describe_number(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else if n <= 10 {
        "small positive"
    } else {
        "large positive"
    }
}

fn grade_to_description(grade: char) -> &'static str {
    match grade {
        'A' => "Excellent",
        'B' => "Good",
        'C' => "Average",
        'D' => "Below Average",
        'F' => "Failing",
        _ => "Invalid grade",
    }
}

fn main() {
    println!("is_even: {} {} {}", is_even(0), is_even(1), is_even(10));
    println!("max: {} {} {}", max(0, 1), max(10, 5), max(10, 100));
    println!(
        "describe_number: {} {} {} {}",
        describe_number(-1),
        describe_number(0),
        describe_number(5),
        describe_number(100)
    );
    println!(
        "grade_to_description: {} {} {} {} {} {}",
        grade_to_description('A'),
        grade_to_description('B'),
        grade_to_description('C'),
        grade_to_description('D'),
        grade_to_description('F'),
        grade_to_description('a')
    );
}

#[cfg(test)]
mod tests {
    use super::*;  // Import functions from parent module

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-2), true);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 10), 10);
        assert_eq!(max(10, 5), 10);
        assert_eq!(max(-3, -8), -3);
        assert_eq!(max(7, 7), 7);
    }

    #[test]
    fn test_describe_number() {
        assert_eq!(describe_number(-5), "negative");
        assert_eq!(describe_number(0), "zero");
        assert_eq!(describe_number(7), "small positive");
        assert_eq!(describe_number(100), "large positive");
    }

    #[test]
    fn test_grade_to_description() {
        assert_eq!(grade_to_description('A'), "Excellent");
        assert_eq!(grade_to_description('B'), "Good");
        assert_eq!(grade_to_description('C'), "Average");
        assert_eq!(grade_to_description('D'), "Below Average");
        assert_eq!(grade_to_description('F'), "Failing");
        assert_eq!(grade_to_description('Z'), "Invalid grade");
    }
}

