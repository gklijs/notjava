use crate::no_package::NoClass;
use warp::Filter;

mod no_package;

#[tokio::main]
async fn main() {
    let hello = warp::path!("times" / u128)
        .map(|additions| format!("Result, {}", NoClass::times(additions)));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}