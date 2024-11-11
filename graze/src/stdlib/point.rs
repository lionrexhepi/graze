use super::{Scalar, Vector};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub(super) x: Scalar,
    pub(super) y: Scalar,
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Vector> for Point {
    fn from(v: Vector) -> Self {
        Point { x: v.x, y: v.y }
    }
}

pub fn point(x: Scalar, y: Scalar) -> Point {
    Point { x, y }
}

pub fn vector(p: Point) -> Vector {
    Vector { x: p.x, y: p.y }
}
