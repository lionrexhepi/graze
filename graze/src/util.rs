#[cfg(test)]
pub mod test_helpers {
    use crate::{
        runtime::{Error, Stack, Value},
        stdlib::{Point, Scalar, Vector},
    };

    #[track_caller]
    pub fn assert_values_eq(actual: Result<Value, Error>, expected: Value) {
        assert_eq!(actual, Ok(expected));
    }

    pub fn dummy_stack<const N: usize>(values: [Value; N]) -> Stack {
        let mut stack = Stack::default();
        for value in values {
            stack.push(value);
        }
        stack
    }

    pub fn scalar<T>(value: T) -> Value
    where
        T: Into<Scalar>,
    {
        Value::Scalar(value.into())
    }

    pub fn vector<T>(x: T, y: T) -> Value
    where
        T: Into<Scalar>,
    {
        Value::Vector(Vector {
            x: x.into(),
            y: y.into(),
        })
    }

    pub fn point<T>(x: T, y: T) -> Value
    where
        T: Into<Scalar>,
    {
        Value::Point(Point {
            x: x.into(),
            y: y.into(),
        })
    }
}
