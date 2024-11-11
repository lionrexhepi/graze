use std::ops::*;

use crate::{
    reverse_pop,
    runtime::{Error, Runtime, Stack, Value},
    token::Number,
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Scalar(ScalarInner);

#[derive(Debug, Clone, Copy)]
enum ScalarInner {
    Integer(i64),
    Float(f64),
}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        match (self.0, rhs.0) {
            (ScalarInner::Integer(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Integer(a + b))
            }
            (ScalarInner::Float(a), ScalarInner::Float(b)) => Scalar(ScalarInner::Float(a + b)),
            (ScalarInner::Integer(a), ScalarInner::Float(b)) => {
                Scalar(ScalarInner::Float(a as f64 + b))
            }
            (ScalarInner::Float(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Float(a + b as f64))
            }
        }
    }
}

impl Sub<Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        match (self.0, rhs.0) {
            (ScalarInner::Integer(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Integer(a - b))
            }
            (ScalarInner::Float(a), ScalarInner::Float(b)) => Scalar(ScalarInner::Float(a - b)),
            (ScalarInner::Integer(a), ScalarInner::Float(b)) => {
                Scalar(ScalarInner::Float(a as f64 - b))
            }
            (ScalarInner::Float(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Float(a - b as f64))
            }
        }
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        match (self.0, rhs.0) {
            (ScalarInner::Integer(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Integer(a * b))
            }
            (ScalarInner::Float(a), ScalarInner::Float(b)) => Scalar(ScalarInner::Float(a * b)),
            (ScalarInner::Integer(a), ScalarInner::Float(b)) => {
                Scalar(ScalarInner::Float(a as f64 * b))
            }
            (ScalarInner::Float(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Float(a * b as f64))
            }
        }
    }
}

impl Div<Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        match (self.0, rhs.0) {
            (ScalarInner::Integer(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Integer(a / b))
            }
            (ScalarInner::Float(a), ScalarInner::Float(b)) => Scalar(ScalarInner::Float(a / b)),
            (ScalarInner::Integer(a), ScalarInner::Float(b)) => {
                Scalar(ScalarInner::Float(a as f64 / b))
            }
            (ScalarInner::Float(a), ScalarInner::Integer(b)) => {
                Scalar(ScalarInner::Float(a / b as f64))
            }
        }
    }
}

impl From<i64> for Scalar {
    fn from(value: i64) -> Self {
        Scalar(ScalarInner::Integer(value))
    }
}

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Scalar(ScalarInner::Float(value))
    }
}

impl From<Scalar> for f64 {
    fn from(value: Scalar) -> f64 {
        match value.0 {
            ScalarInner::Integer(i) => i as f64,
            ScalarInner::Float(f) => f,
        }
    }
}

impl From<Scalar> for i64 {
    fn from(value: Scalar) -> i64 {
        match value.0 {
            ScalarInner::Integer(i) => i,
            ScalarInner::Float(f) => f as i64,
        }
    }
}

impl TryFrom<Number> for Scalar {
    type Error = Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::Integer(i) => {
                if i > i64::MAX as u64 {
                    Err(Error::IntLiteralTooLarge)
                } else {
                    Ok(Scalar(ScalarInner::Integer(i as i64)))
                }
            }
            Number::Float(f) => Ok(Scalar(ScalarInner::Float(f))),
        }
    }
}

impl Scalar {
    pub fn sqrt(self) -> Self {
        match self.0 {
            ScalarInner::Integer(i) => Scalar(ScalarInner::Float((i as f64).sqrt())),
            ScalarInner::Float(f) => Scalar(ScalarInner::Float(f.sqrt())),
        }
    }
}

pub fn sqrt(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => x);
    match x {
        Value::Scalar(scalar) => {
            if f64::from(scalar) < 0.0 {
                Ok(Value::Scalar(scalar.sqrt()))
            } else {
                Err(Error::NonRealResult)
            }
        }
        _ => Err(Error::TypeError),
    }
}
pub fn register(runtime: &mut Runtime) {
    runtime.define_fn("sqrt", sqrt)
}
