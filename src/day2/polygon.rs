use std::{ops::Add, f64::consts::PI};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x, y
        }
    }
    fn magnitude(self) -> f64 {
        f64::from(self.x * self.x + self.y * self.y).sqrt()
    }

    fn dist(self, p2: Point) -> f64 {
        let temp: f64 = ((self.x - p2.x).pow(2) + (self.y - p2.y).pow(2)).into();
        temp.sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, p2: Point) -> Point {
        Point {
            x: self.x + p2.x,
            y: self.y + p2.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon { points: Vec::new() }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    pub fn len(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let mut result = 0.0;
        let mut last_point = self.points[0];
        for curr in &self.points[1..] {
            result += last_point.dist(*curr);
            last_point = *curr;
        }
        result += last_point.dist(self.points[0]);
        result
    }
}

pub struct Circle {
    p: Point,
    len: i32,
}

impl Circle {
    pub fn new(p: Point, len: i32) -> Circle {
        Circle { p, len }
    }
    fn circumference(&self) -> f64 {
        2.0 * f64::from(self.len) * PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    pub fn circumference(&self) -> f64 {
        match self {
            Shape::Circle(circle) => circle.circumference(),
            Shape::Polygon(p) => p.len(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

fn main() {}
