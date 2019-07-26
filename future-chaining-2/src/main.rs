use futures::future;
use tokio;

use hyper::rt::{Future, Stream};
use hyper::{Body, Method, Request};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;

// This example shows how to chain Futures inside a single Impl block
// FIXME: it is broken due do a lifetime error: can it be fixed?

struct MyClient {}

impl MyClient {
    pub fn new() -> MyClient {
        MyClient {}
    }

    pub fn all_requests(&self) -> impl Future<Item = String, Error = ()> {
        let f1_res = self.request_one();
        let final_response = f1_res
            .map(|resp1| {
                eprintln!("req one resp: {}", resp1);
                resp1
            })
            .and_then(move |resp1| {
                self.request_two(resp1).and_then(|resp2| {
                    eprintln!("req two resp: {}", resp2);
                    future::ok(resp2)
                })
            });
        final_response
    }

    fn request_one(&self) -> impl Future<Item = String, Error = ()> {
        eprintln!("Request one");
        let url: Uri = "https://httpbin.org/post".parse().unwrap();
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);
        let body = Body::from("key=val");

        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .body(body)
            .expect("Failed to build request body");
        let fut = client.request(req);

        fut.and_then(|res| {
            eprintln!("Got {}", res.status());
            let resp_data = res.into_body().concat2();
            resp_data
        })
        .map_err(|err| eprintln!("error: {}", err))
        .map(|body| format!("request one succeded: {:?}", body))
    }

    fn request_two(&self, param: String) -> impl Future<Item = String, Error = ()> {
        eprintln!("Request two: {}", param);
        let url: Uri = "https://httpbin.org/get".parse().unwrap();

        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);

        eprintln!("contacting url: {}", url);
        let req = Request::builder()
            .uri(url)
            .body(Body::empty())
            .expect("Failed to build request body");
        let fut = client.request(req);

        fut.and_then(|res| {
            eprintln!("Got {}", res.status());
            let resp_data = res.into_body().concat2();
            resp_data
        })
        .map_err(|err| eprintln!("error: {}", err))
        .map(|body| format!("request two succeded: {:?}", body))
    }
}

fn main() {
    let c = MyClient::new();
    let f = c.all_requests().map(|x| {
        println!("future resolved: {}", x);
        ()
    });
    tokio::run(f);
}
