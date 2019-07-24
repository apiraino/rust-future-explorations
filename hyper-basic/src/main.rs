// Notice we are using the future crate re-exported from hyper
// use futures::future;
use hyper::rt::Future;

use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};

fn main() {
    println!("Start");

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {
            // This is the `Service` that will handle the connection.
            // `service_fn_ok` is a helper to convert a function that
            // returns a Response into a `Service`.
            service_fn_ok(move |_: Request<Body>| Response::new(Body::from("Hello World!\n")))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    // runs on tokio runtime
    println!("Listening on http://{}", addr);
    hyper::rt::run(server);

    println!("Exiting");
}
