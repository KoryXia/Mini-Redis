use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::Notify;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            println!("Waker");
            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(Duration::from_millis(350));
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    // let when = Instant::now() + Duration::from_millis(1000);
    // let future = Delay { when };
    // let out = future.await;
    // println!("{}", out);
    let when = Instant::now() + Duration::from_millis(1000);
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();
    thread::spawn(move || {
        let now = Instant::now();
        if now < when {
            thread::sleep(Duration::from_millis(350));
        }
        notify_clone.notify_one();
    });
    print!("notified");
    notify.notified().await;
}
