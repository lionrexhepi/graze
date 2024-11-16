use std::ops::{Add, Div, Mul, Sub};

use crate::{
    reverse_pop,
    runtime::{Error, Runtime, Stack, Value},
};

use super::Scalar;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: Scalar,
    pub y: Scalar,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Scalar> for Vector {
    type Output = Vector;

    fn div(self, rhs: Scalar) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

pub fn dot(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => lhs, rhs);

    let (Value::Vector(lhs), Value::Vector(rhs)) = (lhs, rhs) else {
        return Err(Error::TypeError);
    };

    Ok(Value::Scalar(lhs.x * rhs.x + lhs.y * rhs.y))
}

pub fn vec2(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => x, y);
    let result = match (x, y) {
        (Value::Scalar(x), Value::Scalar(y)) => Value::Vector(Vector { x, y }),

        _ => return Err(Error::TypeError),
    };

    Ok(result)
}

pub fn register<Backend>(runtime: &mut Runtime<Backend>) {
    runtime.define_fn("dot", dot);
    runtime.define_fn("vec2", vec2);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::test_helpers::*;

    #[test]
    fn test_dot() {
        #[rustfmt::skip]
        let mut stack = dummy_stack([

            vector(1, 2), vector(3, 4),
        ]);

        assert_values_eq(dot(&mut stack), scalar(11));
    }

    #[test]
    fn test_vec2() {
        #[rustfmt::skip]
        let mut stack = dummy_stack([
            scalar(1), scalar(2),
        ]);

        assert_values_eq(vec2(&mut stack), vector(1, 2));
    }
}
