use futures::future::Future;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

fn svc_wait(t: u64) -> impl Future<Item = i32, Error = ()> {
    println!("[start] waiting...");
    let when = Instant::now() + Duration::from_millis(t);
    Delay::new(when)
        .map_err(|e| panic!("timer failed; err={:?}", e))
        .and_then(|_| {
            println!("[end] waiting");
            Ok(42)
        })
}

fn main() {
    let f = svc_wait(1000).map(|_| {
        println!("future finished");
        ()
    });
    tokio::run(f);
}

#[cfg(test)]
mod tests {

    use super::svc_wait;
    use futures::future::Future;
    use tokio;

    #[test]
    fn test_1() {
        let f = svc_wait(500).map(|ret_val| {
            println!("future finished");
            ret_val
        });
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        assert_eq!(42, rt.block_on(f).unwrap());
    }

}
