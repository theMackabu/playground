pub use mini_tokio_attr::main;

use crossbeam::channel;
use futures::future::BoxFuture;
use futures::task::{self, ArcWake};

use std::{
    cell::RefCell,
    future::Future,
    sync::{Arc, Mutex},
    task::Context,
};

pub struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    pub fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();

        MiniTokio { scheduled, sender }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    pub fn run(&self) {
        CURRENT.with(|cell| {
            *cell.borrow_mut() = Some(self.sender.clone());
        });

        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    CURRENT.with(|cell| {
        let borrow = cell.borrow();
        let sender = borrow.as_ref().unwrap();
        Task::spawn(future, sender);
    });
}

thread_local! {
    static CURRENT: RefCell<Option<channel::Sender<Arc<Task>>>> =
        RefCell::new(None);
}

struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }

    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) { let _ = arc_self.executor.send(arc_self.clone()); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::{Duration, Instant},
    };

    pub async fn delay(dur: Duration) {
        struct Delay {
            when: Instant,

            waker: Option<Arc<Mutex<Waker>>>,
        }

        impl Future for Delay {
            type Output = ();

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                if let Some(waker) = &self.waker {
                    let mut waker = waker.lock().unwrap();

                    if !waker.will_wake(cx.waker()) {
                        *waker = cx.waker().clone();
                    }
                } else {
                    let when = self.when;
                    let waker = Arc::new(Mutex::new(cx.waker().clone()));
                    self.waker = Some(waker.clone());

                    thread::spawn(move || {
                        let now = Instant::now();

                        if now < when {
                            thread::sleep(when - now);
                        }

                        let waker = waker.lock().unwrap();
                        waker.wake_by_ref();
                    });
                }

                if Instant::now() >= self.when {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }

        let future = Delay {
            when: Instant::now() + dur,
            waker: None,
        };

        future.await;
    }

    pub async fn async_function() -> u32 {
        delay(Duration::from_millis(100)).await;
        42
    }

    #[test]
    fn test_runtime() {
        use crate::tests::delay;

        let mini_tokio = MiniTokio::new();

        mini_tokio.spawn(async {
            spawn(async {
                delay(Duration::from_millis(100)).await;
                println!("world");
            });

            spawn(async {
                println!("hello");
            });

            delay(Duration::from_millis(200)).await;
            std::process::exit(0);
        });

        mini_tokio.run();
    }

    #[test]
    fn test_proc_macro_main() {
        #[main]
        async fn test_main() {
            let value = async_function().await;
            assert_eq!(value, 42);
        }

        test_main();
    }
}
