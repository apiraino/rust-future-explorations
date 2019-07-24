use futures::prelude::*;

struct SleepingFuture {
    name: String,
    is_done: bool,
}

impl SleepingFuture {
    fn new(name: String, is_done: bool) -> SleepingFuture {
        SleepingFuture { name, is_done }
    }
}

impl Future for SleepingFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // Ok(Async::Ready(()))
        Ok(Async::NotReady)
    }

    // fn _poll(&mut self) -> Poll<Self::Item, Self::Error> {
    //     match self.is_done {
    //         // Ok(Async::Ready(curr)) => {
    //         true => {
    //             eprintln!("Future {} is Ready", self.name);
    //             Ok(Async::Ready(()))
    //         }
    //         // Ok(Async::NotReady) => {
    //         false => {
    //             eprintln!("Future {} is NotReady ...", self.name);
    //             Ok(Async::NotReady)
    //         }
    //     }
    // }
}

struct AwakeFuture {
    name: String,
    count: i32,
}

impl AwakeFuture {
    fn new(name: String) -> AwakeFuture {
        AwakeFuture { name, count: 0 }
    }
}

impl Future for AwakeFuture {
    type Item = i32;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.count {
            // Ok(Async::Ready(curr)) => {
            3 => {
                eprintln!(
                    "[{}] Future {} has finished counting",
                    self.count, self.name
                );
                Ok(Async::Ready(self.count))
            }

            // Ok(Async::NotReady) => {
            _ => {
                eprintln!("[{}] Future {} is not yet ready ...", self.count, self.name);
                // self.count += 1;
                Ok(Async::NotReady)
            }
        }
        // Ok(Async::Ready(()))
        // Ok(Async::NotReady)
    }
}

fn main() {
    // this future will hang indefinitively
    let sleeping_future = SleepingFuture::new(String::from("sleeping-future"), false);

    // this future will resolve immediately
    let awake_future = AwakeFuture::new(String::from("awake-future"));

    // let both = o.join(o1);
    // let runner = both.map(|(x, y)| {
    //     eprint!("x={:?} y={:?}", x, y);
    //     ()
    // });

    tokio::run(awake_future.map(|x| {
        eprint!("x={:?}", x);
        ()
    }));
}
