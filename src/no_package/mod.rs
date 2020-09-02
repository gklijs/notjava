use core::fmt;
use rand::Rng;

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
}

// Implement `Display` for `NoClass`.
impl fmt::Display for NoClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "just two random numbers x:{} and y:{}", self.x, self.y)
    }
}
