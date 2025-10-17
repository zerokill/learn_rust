struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point(x, y)
    }

    fn distance_from_origin(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2)) as f64).sqrt()
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        self.width + self.width + self.height + self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, to: &Rectangle) -> bool {
        to.width <= self.width && to.height <= self.height
    }

    fn scale(&mut self, scale: u32) {
        self.width *= scale;
        self.height *= scale;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_new() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn test_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_perimeter() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.perimeter(), 60);
    }

    #[test]
    fn test_is_square() {
        let rect = Rectangle::new(10, 20);
        let square = Rectangle::new(10, 10);
        assert_eq!(rect.is_square(), false);
        assert_eq!(square.is_square(), true);
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle::new(10, 20);
        let rect2 = Rectangle::new(5, 10);
        let rect3 = Rectangle::new(15, 25);

        assert_eq!(rect1.can_hold(&rect2), true);
        assert_eq!(rect1.can_hold(&rect3), false);
    }

    #[test]
    fn test_scale() {
        let mut rect = Rectangle::new(10, 20);
        rect.scale(2);
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 40);
    }

    #[test]
    fn test_point() {
        let p = Point::new(3, 4);
        assert_eq!(p.0, 3);
        assert_eq!(p.1, 4);
        assert_eq!(p.distance_from_origin(), 5.0);
    }
}
