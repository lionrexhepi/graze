use crate::{
    reverse_pop,
    runtime::{Error, Runtime, Stack, Value},
};

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

fn pnt2(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => x, y);
    let (Value::Scalar(x), Value::Scalar(y)) = (x, y) else {
        return Err(Error::TypeError);
    };
    Ok(Value::Point(Point { x, y }))
}

fn lvec(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => pnt);
    let Value::Point(pnt) = pnt else {
        return Err(Error::TypeError);
    };
    Ok(Value::Vector(Vector { x: pnt.x, y: pnt.y }))
}

fn x(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => pnt);
    match pnt {
        Value::Point(pnt) => Ok(Value::Scalar(pnt.x)),
        Value::Vector(vec) => Ok(Value::Scalar(vec.x)),
        _ => Err(Error::TypeError),
    }
}

fn y(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => pnt);
    match pnt {
        Value::Point(pnt) => Ok(Value::Scalar(pnt.y)),
        Value::Vector(vec) => Ok(Value::Scalar(vec.y)),
        _ => Err(Error::TypeError),
    }
}

pub fn register_stdlib(runtime: &mut Runtime) {
    runtime.register("pnt2", pnt2);
    runtime.register("lvec", lvec);
    runtime.register("x", x);
    runtime.register("y", y);
}
