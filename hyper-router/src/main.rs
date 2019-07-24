use std::time::{Duration, Instant};

use tokio::timer::Delay;

use futures::future;

use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::Client;
use hyper::{Body, Method, Request, Response, Server, Uri};

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct RespStruct {
    // only partially parsing the response (yes, I'm lazy)
    origin: String,
    url: String,
}

fn fetch_data() -> impl Future<Item = future::FutureResult<RespStruct, String>, Error = ()> {
    let uri: Uri = "http://httpbin.org/get".parse().expect("Cannot parse URL");
    Client::new()
        .get(uri)
        // Future is polled here
        .and_then(|res| {
            println!("got status: {:?}", res.status());

            // here you can use the body to write it to a file
            // or return it via Ok()
            // If you return it via for example Ok(users)
            // then you need to adjust the return type in impl Future<Item=TYPE
            // Examples can be found here:
            // https://github.com/gruberb/futures_playground

            // Response is this
            // pub struct Response {
            //     status: StatusCode,
            //     headers: HeaderMap,
            //     url: Box<Url>,
            //     body: Decoder,
            //     ...
            // }
            // need to get the body as Decoder

            // create a Vec<u8>
            // hyper::body::chunk::Chunk
            res.into_body().concat2()
        })
        .map_err(|err| println!("error: {}", err))
        .map(|body| {
            // here parse the FutureResult
            // serialize into a validated Struct
            let decoded: RespStruct = serde_json::from_slice(&body).expect("Couldn't deserialize");
            future::ok(decoded)
        })
}

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

// Just an alias to save us some typing
type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn service_router(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());
    eprintln!("{} {}", req.method(), req.uri().path());

    // routes the request to the appropriate worker
    match (req.method(), req.uri().path()) {
        // GET /wait
        (&Method::GET, "/wait") => {
            let r = svc_wait(1500);
            hyper::rt::spawn(r);
            *response.body_mut() = Body::from(format!("Triggered waiting {}ms", 1500));
        }

        // unrelated: for linting these warning see: https://github.com/rust-lang/rust/issues/62411
        // and linked issues
        (&Method::GET, "/fetch") => {
            let r = fetch_data().map(|x| {
                println!("got data: {:?}", x);
            });
            hyper::rt::spawn(r);
            *response.body_mut() = Body::from("Sent request to external webservice");
        }

        _ => {
            *response.body_mut() =
                Body::from(format!("{}{} not managed", req.method(), req.uri().path()));
        }
    }
    eprintln!("Returning a response");
    Box::new(future::ok(response))
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {
            // now we spawn a Future with our request router
            service_fn(service_router)
        })
        .map_err(|e| eprintln!("server error: {}", e));

    // runs on tokio runtime
    println!("Listening on http://{}", addr);
    hyper::rt::run(server);

    println!("Exiting");
}
