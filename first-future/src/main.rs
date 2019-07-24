use futures::future;
use futures::future::Future;

fn my_fut() -> impl Future<Item = (), Error = ()> {
    println!("running my_fut");
    future::ok(())
}

fn main() {
    // Run the Future, it returns a unit ()
    tokio::run(my_fut());
}
