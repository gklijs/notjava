use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    println!("Two random floats, {} and {}", x, y);
}
