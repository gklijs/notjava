use rand::Rng;
use core::fmt;

#[derive(Debug)]
struct NoClass{
    x: f64,
    y: f64,
}

// Implement `Display` for `NoClass`.
impl fmt::Display for NoClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "just two random numbers x:{} and y:{}", self.x, self.y)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    let no_class = NoClass{x,y};
    println!("No class: {}", no_class);
}
