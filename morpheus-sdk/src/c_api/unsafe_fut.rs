use std::{
    future::Future,
    marker::Unpin,
    pin::Pin,
    task::{Context, Poll},
};

pub struct UnsafeSendFuture<T: Future + Unpin>(T);

impl<T: Future + Unpin> From<T> for UnsafeSendFuture<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

unsafe impl<T: Future + Unpin> Send for UnsafeSendFuture<T> {}

impl<T: Future + Unpin> Unpin for UnsafeSendFuture<T> {}

impl<T: Future + Unpin> Future for UnsafeSendFuture<T> {
    type Output = T::Output;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T::Output> {
        T::poll(Pin::new(&mut self.0), cx)
    }
}
