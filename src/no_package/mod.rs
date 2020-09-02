use core::fmt;
use rand::Rng;
use std::ops::Add;

#[derive(Debug)]
pub struct NoClass {
    x: f64,
    y: f64,
}

impl NoClass {
    pub fn new() -> NoClass {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        NoClass { x, y }
    }

    pub fn times(additions: u128) -> NoClass{
        let mut result = NoClass::new();
        for _ in 1..additions {
            result = result + NoClass::new()
        };
        result
    }
}

impl fmt::Display for NoClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "just two random numbers x:{} and y:{}", self.x, self.y)
    }
}

impl Add for NoClass {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
