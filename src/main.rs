use crate::no_package::NoClass;

mod no_package;

fn main() {
    println!("Two no class added: {}", NoClass::new() + NoClass::new());
}
