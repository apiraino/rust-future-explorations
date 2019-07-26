use futures::{future, Future};
use tokio;

// This example shows how to chain Futures together
// src: StackOverflow
// TODO: can be improved

fn add_one(x: i64) -> impl Future<Item = i64, Error = ()> {
    future::ok(x).map(|x| x + 1)
}

fn double(x: i64) -> impl Future<Item = i64, Error = ()> {
    future::ok(x).map(|x| x * 2)
}

fn add_one_then_double(x: i64) -> impl Future<Item = i64, Error = ()> {
    future::ok(x).and_then(add_one).and_then(double)
}

fn main() {
    let f = add_one_then_double(10).map(|x| {
        println!("future resolved: {}", x);
        ()
    });
    tokio::run(f);
}
