use std::time::{Duration, Instant};

use tokio::timer::Delay;

use futures::future::Future;

fn svc_wait(t: u64) -> impl Future<Item = (), Error = ()> {
    println!("[start] waiting...");
    let when = Instant::now() + Duration::from_millis(t);
    Delay::new(when)
        .map_err(|e| panic!("timer failed; err={:?}", e))
        .and_then(|_| {
            println!("[end] waiting");
            Ok(())
        })
}

fn main() {
    // Future returns a ()
    let f = svc_wait(1000).map(|_| {
        println!("future finished");
        ()
    });
    tokio::run(f);
}
