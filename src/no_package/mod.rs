use core::fmt;

#[derive(Debug)]
pub (crate) struct NoClass{
    pub (crate)x: f64,
    pub (crate)y: f64,
}

// Implement `Display` for `NoClass`.
impl fmt::Display for NoClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "just two random numbers x:{} and y:{}", self.x, self.y)
    }
}