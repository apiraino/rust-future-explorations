use std::time::{Duration, Instant};

use tokio::timer::Delay;

use hyper::rt::Future;

use hyper::service::service_fn_ok;
use hyper::Server;
use hyper::{Body, Request, Response};

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
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {
            service_fn_ok(|req: Request<Body>| {
                // received the client connection
                eprintln!("Received client: {:?}", req.headers());
                // creating the future
                let f = svc_wait(2000);
                // the future is run NOW
                hyper::rt::spawn(f);
                // the client receives immediately a reply
                eprintln!("Sending back NOW a response to the client");
                Response::new(Body::from("Future triggered"))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    // runs on tokio runtime
    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
