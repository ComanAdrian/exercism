use std::ops::{Add, Sub};

pub struct Triangle<T>
where
    T: Numeric,
{
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Numeric,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|&s| s == Numeric::zero()) || !Triangle::validate(sides) {
            return None;
        }

        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[0] != self.sides[2]
            && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }

    fn validate(sides: [T; 3]) -> bool {
        let first = (sides[0] + sides[1]) >= sides[2];
        let second = (sides[0] + sides[2]) >= sides[1];
        let third = (sides[1] + sides[2]) >= sides[0];
        first && second && third
    }
}

// todo: Refactor, see fizzy
pub trait Numeric: PartialEq + Add<Output = Self> + PartialOrd + Sub<Output = Self> + Copy {
    fn zero() -> Self;
}

impl Numeric for u32 {
    fn zero() -> Self {
        0
    }
}

impl Numeric for f64 {
    fn zero() -> Self {
        0.0
    }
}
