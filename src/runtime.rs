// runtime.rs
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::cell::RefCell;
use std::rc::Rc;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

pub struct JoinHandle<T> {
    future: Rc<RefCell<BoxFuture<T>>>,
}

impl<T> Future for JoinHandle<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        self.future.borrow_mut().as_mut().poll(cx)
    }
}

pub struct MiniRuntime {
    tasks: Vec<BoxFuture<()>>,
}

impl MiniRuntime {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn block_on<F: Future>(&mut self, future: F) -> F::Output {
        let mut main_future = Box::pin(future);
        
        const VTABLE: RawWakerVTable = RawWakerVTable::new(
            |_| RAW_WAKER,
            |_| (), 
            |_| (),
            |_| (),
        );
        const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(RAW_WAKER) };
        let mut context = Context::from_waker(&waker);

        loop {
            if let Poll::Ready(output) = main_future.as_mut().poll(&mut context) {
                return output;
            }

            let mut i = 0;
            while i < self.tasks.len() {
                if let Poll::Ready(()) = self.tasks[i].as_mut().poll(&mut context) {
                    let _completed_task = self.tasks.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
}

thread_local! {
    static RUNTIME: RefCell<MiniRuntime> = RefCell::new(MiniRuntime::new());
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    let boxed_future: BoxFuture<F::Output> = Box::pin(future);
    let future_rc = Rc::new(RefCell::new(boxed_future));
    let handle = JoinHandle {
        future: future_rc.clone(),
    };

    let task_future: BoxFuture<()> = Box::pin(async move {
        let _ = JoinHandle { future: future_rc }.await;
    });

    RUNTIME.with(|rt| rt.borrow_mut().tasks.push(task_future));
    handle
}