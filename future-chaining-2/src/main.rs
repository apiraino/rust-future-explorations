use futures::future;
use tokio;

use hyper::rt::{Future, Stream};
use hyper::{client::HttpConnector, Client, Uri};
use hyper::{Body, Method, Request};
use hyper_tls::HttpsConnector;

// This example shows how to chain Futures inside a single Impl block

// The trick here is to make che http client clonable
// and clone it before running the second future.
// all_requests() takes ownership of the http client (&self) and consume it,
// so it won't available to the second nested future

#[derive(Clone)]
struct MyClient {
    client: Client<HttpsConnector<HttpConnector>, hyper::Body>,
}

impl MyClient {
    pub fn new() -> MyClient {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);
        MyClient { client }
    }

    pub fn all_requests(&self) -> impl Future<Item = String, Error = ()> {
        let f1_res = self.request_one();
        // Notice: cloning the whole client, the second future will use it
        let client_cloned = self.clone();
        let final_response = f1_res
            .map(|resp_future_one| {
                eprintln!("future one returned: {}", resp_future_one);
                resp_future_one
            })
            .and_then(move |resp1| {
                client_cloned
                    .request_two(resp1)
                    .and_then(|resp_future_two| {
                        eprintln!("future two returned: {}", resp_future_two);
                        future::ok(resp_future_two)
                    })
            });
        final_response
    }

    fn request_one(&self) -> impl Future<Item = String, Error = ()> {
        eprintln!("Request one");
        let url: Uri = "https://httpbin.org/post".parse().unwrap();
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);

        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .body(Body::from("key=val"))
            .expect("Failed to build request body");
        let fut = client.request(req);

        fut.and_then(|res| {
            eprintln!("Request one: got {}", res.status());
            let resp_data = res.into_body().concat2();
            resp_data
        })
        .map_err(|err| eprintln!("error: {}", err))
        .map(|body| format!("{:?}", body))
    }

    fn request_two(&self, param: String) -> impl Future<Item = String, Error = ()> {
        eprintln!("Request two got params: {:?}", param);
        let url: Uri = "https://httpbin.org/get".parse().unwrap();
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);

        let req = Request::builder()
            .uri(url)
            .body(Body::empty())
            .expect("Failed to build request body");
        let fut = client.request(req);

        fut.and_then(|res| {
            eprintln!("Request two: got {}", res.status());
            let resp_data = res.into_body().concat2();
            resp_data
        })
        .map_err(|err| eprintln!("error: {}", err))
        .map(|body| format!("{:?}", body))
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
