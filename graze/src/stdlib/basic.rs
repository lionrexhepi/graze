use crate::{
    reverse_pop,
    runtime::{
        Error, Runtime, Stack,
        Value::{self, *},
    },
};

pub fn add(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Scalar(a), Scalar(b)) => Ok(Scalar(a + b)),
        (Vector(a), Vector(b)) => Ok(Vector(a + b)),
        (Vector(vec), Point(pnt)) | (Point(pnt), Vector(vec)) => Ok(Point(pnt + vec)),
        _ => Err(Error::TypeError),
    }
}

pub fn sub(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Scalar(a), Scalar(b)) => Ok(Scalar(a - b)),
        (Vector(a), Vector(b)) => Ok(Vector(a - b)),
        (Point(a), Point(b)) => Ok(Vector(a - b)),
        (Point(pnt), Vector(vec)) => Ok(Point(pnt - vec)),
        _ => Err(Error::TypeError),
    }
}

pub fn mul(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Scalar(a), Scalar(b)) => Ok(Scalar(a * b)),
        (Vector(vec), Scalar(r)) | (Scalar(r), Vector(vec)) => Ok(Vector(vec * r)),
        _ => Err(Error::TypeError),
    }
}

pub fn div(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Scalar(a), Scalar(b)) => Ok(Scalar(a / b)),
        (Vector(vec), Scalar(r)) => Ok(Vector(vec / r)),
        _ => Err(Error::TypeError),
    }
}

pub fn register(runtime: &mut Runtime) {
    runtime.define_fn("add", add);
    runtime.define_fn("sub", sub);
    runtime.define_fn("mul", mul);
    runtime.define_fn("div", div);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::test_helpers::*;

    #[test]
    fn test_add() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
             [
                point(1, 2), point(3, 4),
                point(1, 2), vector(3, 4),
                vector(1, 2), vector(3, 4),
                scalar(1), scalar(2),
            ],
        );

        assert_values_eq(add(&mut stack), scalar(3));
        assert_values_eq(add(&mut stack), vector(4, 6));
        assert_values_eq(add(&mut stack), point(4, 6));

        assert_eq!(add(&mut stack), Err(Error::TypeError));
    }

    #[test]
    fn test_sub() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
            [
                point(1, 2), point(3, 4),
                point(1, 2), vector(3, 4),
                vector(1, 2), vector(3, 4),
                scalar(1), scalar(2),
            ],
        );

        assert_values_eq(sub(&mut stack), scalar(-1));
        assert_values_eq(sub(&mut stack), vector(-2, -2));
        assert_values_eq(sub(&mut stack), point(-2, -2));
        assert_values_eq(sub(&mut stack), vector(-2, -2));
    }

    #[test]
    fn test_mul() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
            [
                vector(1, 2), scalar(3),
                scalar(1), vector(3, 4),
                scalar(1), scalar(2),
            ],
        );

        assert_values_eq(mul(&mut stack), scalar(2));
        assert_values_eq(mul(&mut stack), vector(3, 4));
        assert_values_eq(mul(&mut stack), vector(3, 6));
    }

    #[test]
    fn test_div() {
        #[rustfmt::skip]
        let mut stack = dummy_stack(
            [
                scalar(1), vector(3, 4),
                vector(1, 2), scalar(3),
                scalar(1), scalar(2),
            ],
        );

        assert_values_eq(div(&mut stack), scalar(0.5));
        assert_values_eq(div(&mut stack), vector(1.0 / 3.0, 2.0 / 3.0));
        assert_eq!(div(&mut stack), Err(Error::TypeError));
    }
}
