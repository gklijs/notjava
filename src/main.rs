mod no_package;

use rand::Rng;
use crate::no_package::NoClass;

fn main() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    let no_class = NoClass{x,y};
    println!("No class: {}", no_class);
}
