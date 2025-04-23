// time.rs
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub async fn sleep(duration: Duration) {
    let deadline = Instant::now() + duration;
    Sleep { deadline }.await
}

struct Sleep {
    deadline: Instant,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.deadline {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}