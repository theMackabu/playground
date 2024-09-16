use crossbeam::channel;

use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll, Wake, Waker},
    time::{Duration, Instant},
};

pub use mini_tokio_attr::main;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

thread_local! {
    static CURRENT: RefCell<Option<Runtime>> = RefCell::new(None);
}

pub struct Runtime {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
    task_count: Arc<AtomicUsize>,
}

struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    executor: channel::Sender<Arc<Task>>,
    task_count: Arc<AtomicUsize>,
}

impl Runtime {
    pub fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        let task_count = Arc::new(AtomicUsize::new(0));
        Runtime { scheduled, sender, task_count }
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        CURRENT.with(|cell| {
            if cell.borrow().is_some() {
                panic!("Attempting to start a runtime from within a runtime");
            }
            *cell.borrow_mut() = Some(self.clone());
        });

        let (output_sender, output_receiver) = channel::bounded(1);
        let wrapped = SpawnableFuture::new(future, output_sender);
        let main_task = Arc::new(Task {
            future: Mutex::new(Box::pin(wrapped)),
            executor: self.sender.clone(),
            task_count: self.task_count.clone(),
        });

        self.task_count.fetch_add(1, Ordering::SeqCst);
        let _ = self.sender.send(main_task);

        loop {
            if let Ok(task) = self.scheduled.try_recv() {
                task.poll();
            }

            if let Ok(output) = output_receiver.try_recv() {
                // Ensure all spawned tasks are completed
                while self.task_count.load(Ordering::SeqCst) > 1 {
                    if let Ok(task) = self.scheduled.try_recv() {
                        task.poll();
                    }
                }

                CURRENT.with(|cell| {
                    *cell.borrow_mut() = None;
                });

                return output;
            }

            if self.task_count.load(Ordering::SeqCst) == 0 {
                break;
            }
        }

        CURRENT.with(|cell| {
            *cell.borrow_mut() = None;
        });

        panic!("Runtime exited without producing a result");
    }
}

impl Clone for Runtime {
    fn clone(&self) -> Self {
        Runtime {
            scheduled: self.scheduled.clone(),
            sender: self.sender.clone(),
            task_count: self.task_count.clone(),
        }
    }
}

impl Task {
    fn poll(self: Arc<Self>) {
        let waker = TaskWaker::new(self.clone());
        let mut cx = Context::from_waker(&waker);

        if let Ok(mut future) = self.future.try_lock() {
            if future.as_mut().poll(&mut cx).is_ready() {
                self.task_count.fetch_sub(1, Ordering::SeqCst);
            }
        } else {
            let _ = self.executor.send(self.clone());
        }
    }
}

struct TaskWaker {
    task: Arc<Task>,
}

impl TaskWaker {
    fn new(task: Arc<Task>) -> Waker { Waker::from(Arc::new(TaskWaker { task })) }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) { let _ = self.task.executor.send(self.task.clone()); }

    fn wake_by_ref(self: &Arc<Self>) { let _ = self.task.executor.send(self.task.clone()); }
}

struct SpawnableFuture<F: Future> {
    inner: Pin<Box<F>>,
    output_sender: Option<channel::Sender<F::Output>>,
}

impl<F: Future> SpawnableFuture<F> {
    fn new(future: F, output_sender: channel::Sender<F::Output>) -> Self {
        SpawnableFuture {
            inner: Box::pin(future),
            output_sender: Some(output_sender),
        }
    }
}

impl<F: Future + Send + 'static> Future for SpawnableFuture<F>
where
    F::Output: Send + 'static,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut().poll(cx) {
            Poll::Ready(output) => {
                if let Some(sender) = self.output_sender.take() {
                    let _ = sender.send(output);
                }
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub async fn delay(dur: Duration) {
    let start = Instant::now();
    while start.elapsed() < dur {
        yield_now().await;
    }
}

pub async fn yield_now() {
    struct Yield {
        yielded: bool,
    }

    impl Future for Yield {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.yielded {
                Poll::Ready(())
            } else {
                self.yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    Yield { yielded: false }.await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime() {
        let rt = Runtime::new();

        rt.block_on(async {
            delay(Duration::from_millis(500)).await;

            println!("hello");
            println!("world");

            delay(Duration::from_millis(500)).await;
        });

        println!("Runtime exited");
    }

    #[test]
    #[should_panic(expected = "Attempting to start a runtime from within a runtime")]
    fn test_nested_runtime() {
        let rt = Runtime::new();

        rt.block_on(async {
            let inner_rt = Runtime::new();
            inner_rt.block_on(async {
                // This should panic
            });
        });
    }
}
