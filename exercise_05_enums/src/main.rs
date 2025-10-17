enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 50,
        }
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * 2.,
            Shape::Rectangle(width, height) => 2.0 * width + 2.0 * height,
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn find_even(numbers: &[i32]) -> Option<i32> {
    for num in numbers {
        if (num % 2) == 0 {
            return Some(*num);
        }
    }
    None
}

fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 60);
        assert_eq!(TrafficLight::Yellow.duration(), 10);
        assert_eq!(TrafficLight::Green.duration(), 50);
    }

    #[test]
    fn test_shape_circle() {
        let circle = Shape::Circle(5.0);
        assert!((circle.area() - 78.54).abs() < 0.01);
        assert!((circle.perimeter() - 31.42).abs() < 0.01);
    }

    #[test]
    fn test_shape_rectangle() {
        let rect = Shape::Rectangle(4.0, 5.0);
        assert_eq!(rect.area(), 20.0);
        assert_eq!(rect.perimeter(), 18.0);
    }

    #[test]
    fn test_shape_triangle() {
        let triangle = Shape::Triangle(3.0, 4.0, 5.0);
        assert!((triangle.area() - 6.0).abs() < 0.01);
        assert_eq!(triangle.perimeter(), 12.0);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(
            safe_divide(10.0, 0.0),
            Err(String::from("Division by zero"))
        );
    }

    #[test]
    fn test_find_even() {
        assert_eq!(find_even(&[1, 3, 4, 7]), Some(4));
        assert_eq!(find_even(&[1, 3, 5]), None);
        assert_eq!(find_even(&[2, 4, 6]), Some(2));
    }

    #[test]
    fn test_get_first_char() {
        assert_eq!(get_first_char("hello"), Some('h'));
        assert_eq!(get_first_char(""), None);
        assert_eq!(get_first_char("x"), Some('x'));
    }
}
