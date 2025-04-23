// task.rs
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::cell::RefCell;
use std::rc::Rc;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;

pub struct JoinHandle<T> {
    value: Rc<RefCell<Option<T>>>,
}

pub(crate) struct JoinHandleRegistration<T> {
    future: BoxFuture<T>,
    value: Rc<RefCell<Option<T>>>,
}

impl<T> JoinHandle<T> {
    pub(crate) fn new(future: BoxFuture<T>) -> (Self, JoinHandleRegistration<T>) {
        let value = Rc::new(RefCell::new(None));
        let registration = JoinHandleRegistration {
            future,
            value: value.clone(),
        };
        (Self { value }, registration)
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = T;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        if let Some(value) = self.value.borrow_mut().take() {
            Poll::Ready(value)
        } else {
            Poll::Pending
        }
    }
}

impl<T> Future for JoinHandleRegistration<T> {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(value) = self.future.as_mut().poll(cx) {
            *self.value.borrow_mut() = Some(value);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}