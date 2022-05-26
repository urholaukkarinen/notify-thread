#[deny(clippy::all)]
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct JoinHandle<T> {
    inner: std::thread::JoinHandle<T>,
    thread_ctx: ThreadContext,
}

impl<T> Deref for JoinHandle<T> {
    type Target = std::thread::JoinHandle<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> JoinHandle<T> {
    pub fn notify(&self) {
        self.thread_ctx.notify();
    }

    pub fn join(self) -> std::thread::Result<T> {
        self.inner.join()
    }
}

#[derive(Default, Clone)]
pub struct ThreadContext {
    notified: Arc<AtomicBool>,
}

impl ThreadContext {
    fn notify(&self) {
        self.notified.store(true, Ordering::SeqCst)
    }

    pub fn notified(&self) -> bool {
        self.notified.load(Ordering::SeqCst)
    }
}

pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce(ThreadContext) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let ctx = ThreadContext::default();
    let ctx_clone = ctx.clone();

    let join_handle = std::thread::spawn(move || f(ctx_clone));

    JoinHandle {
        inner: join_handle,
        thread_ctx: ctx,
    }
}
