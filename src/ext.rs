use crate::SyncWrapper;
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

pub struct SyncFuture<F> {
    fut: SyncWrapper<F>
}
impl <F> SyncFuture<F> {
    pub fn new(fut: F) -> Self {
        Self { fut: SyncWrapper::new(fut) }
    }
}
impl <F: Future> Future for SyncFuture<F> {
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = unsafe { self.map_unchecked_mut(|x| x.fut.get_mut()) };
        fut.poll(cx)
    }
}

pub struct SyncStream<S> {
    st: SyncWrapper<S>
}
impl <S> SyncStream<S> {
    pub fn new(st: S) -> Self {
        Self { st: SyncWrapper::new(st) }
    }
}
impl <S: futures_core::Stream> futures_core::Stream for SyncStream<S> {
    type Item = S::Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let st = unsafe { self.map_unchecked_mut(|x| x.st.get_mut()) };
        st.poll_next(cx)
    }
}