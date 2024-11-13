use crate::{
    reverse_pop,
    runtime::{Error, Runtime, Stack, Value},
};

use super::{Scalar, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: Scalar,
    pub y: Scalar,
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

pub fn pnt2(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => x, y);
    let (Value::Scalar(x), Value::Scalar(y)) = (x, y) else {
        return Err(Error::TypeError);
    };
    Ok(Value::Point(Point { x, y }))
}

pub fn lvec(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => pnt);
    let Value::Point(pnt) = pnt else {
        return Err(Error::TypeError);
    };
    Ok(Value::Vector(Vector { x: pnt.x, y: pnt.y }))
}

pub fn x(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => pnt);
    match pnt {
        Value::Point(pnt) => Ok(Value::Scalar(pnt.x)),
        Value::Vector(vec) => Ok(Value::Scalar(vec.x)),
        _ => Err(Error::TypeError),
    }
}

pub fn y(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => pnt);
    match pnt {
        Value::Point(pnt) => Ok(Value::Scalar(pnt.y)),
        Value::Vector(vec) => Ok(Value::Scalar(vec.y)),
        _ => Err(Error::TypeError),
    }
}

pub fn jump(stack: &mut Stack) -> Result<Value, Error> {
    let Value::Vector(vec) = super::vector::vec2(stack)? else {
        unreachable!()
    };
    reverse_pop!(stack => previous);
    let Value::Point(previous) = previous else {
        return Err(Error::TypeError);
    };

    Ok(Value::Point(previous + vec))
}

pub fn register(runtime: &mut Runtime) {
    runtime.define_fn("pnt2", pnt2);
    runtime.define_fn("lvec", lvec);
    runtime.define_fn("x", x);
    runtime.define_fn("y", y);
    runtime.define_fn("jump", jump);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stdlib::test_helpers::*;

    #[test]
    fn test_pnt2() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
            [
                scalar(3), vector(4, 5),
                scalar(1), scalar(2),
            ]
        );

        assert_values_eq(pnt2(&mut stack), point(1, 2));
        assert_eq!(pnt2(&mut stack), Err(Error::TypeError))
    }

    #[test]
    fn test_lvec() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
            [
                vector(3, 4),
                point(1, 2),
            ]
        );

        assert_values_eq(lvec(&mut stack), vector(1, 2));
        assert_eq!(lvec(&mut stack), Err(Error::TypeError))
    }

    #[test]
    fn test_x() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
            [
                scalar(1),
                vector(3, 4),
                point(1, 2),
            ]
        );

        assert_values_eq(x(&mut stack), scalar(1));
        assert_values_eq(x(&mut stack), scalar(3));
        assert_eq!(x(&mut stack), Err(Error::TypeError))
    }

    #[test]
    fn test_y() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
            [
                scalar(2),
                vector(3, 4),
                point(1, 2),
            ]
        );

        assert_values_eq(y(&mut stack), scalar(2));
        assert_values_eq(y(&mut stack), scalar(4));
        assert_eq!(y(&mut stack), Err(Error::TypeError))
    }
}
