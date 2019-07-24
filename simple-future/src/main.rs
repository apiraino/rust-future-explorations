use futures::future;
use futures::future::Future;

// The Future returns an Ok(i32)
fn my_fut() -> impl Future<Item = i32, Error = ()> {
    println!("running my_fut");
    future::ok(42)
}

fn main() {
    // Create a Future, check the value when it resolves
    // The Future does not start here.
    let f = my_fut().map(|x| {
        println!("future resolved: {}", x);
        ()
    });

    // run the Future using the Tokio runtime
    // The Future execution starts HERE
    tokio::run(f);
}
