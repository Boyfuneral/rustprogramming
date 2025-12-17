use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

pub enum Task {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Stop, // used to reduce worker count
}

struct State {
    queue: VecDeque<Task>,
    shutdown: bool,
}

pub struct ThreadPool {
    shared: Arc<(Mutex<State>, Condvar)>,
    workers: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_workers: usize) -> Self {
        let shared = Arc::new((
            Mutex::new(State {
                queue: VecDeque::new(),
                shutdown: false,
            }),
            Condvar::new(),
        ));

        let mut pool = Self {
            shared: shared.clone(),
            workers: Vec::new(),
        };
        pool.add_workers(num_workers);
        pool
    }

    pub fn submit<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (lock, cv) = &*self.shared;
        let mut st = lock.lock().unwrap();
        if st.shutdown {
            return;
        }
        st.queue.push_back(Task::Job(Box::new(job)));
        cv.notify_one();
    }

    pub fn add_workers(&mut self, n: usize) {
        for _ in 0..n {
            let shared = self.shared.clone();
            let handle = thread::spawn(move || loop {
                let task = {
                    let (lock, cv) = &*shared;
                    let mut st = lock.lock().unwrap();

                    while st.queue.is_empty() && !st.shutdown {
                        st = cv.wait(st).unwrap();
                    }

                    if st.shutdown {
                        return;
                    }

                    st.queue.pop_front()
                };

                match task {
                    Some(Task::Job(job)) => job(),
                    Some(Task::Stop) => return,
                    None => continue,
                }
            });

            self.workers.push(handle);
        }
    }

    pub fn remove_workers(&mut self, n: usize) {
        let (lock, cv) = &*self.shared;
        let mut st = lock.lock().unwrap();
        for _ in 0..n {
            st.queue.push_back(Task::Stop);
        }
        cv.notify_all();
    }

    pub fn shutdown(&mut self) {
        let (lock, cv) = &*self.shared;
        let mut st = lock.lock().unwrap();
        st.shutdown = true;
        cv.notify_all();
        drop(st);

        while let Some(h) = self.workers.pop() {
            let _ = h.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn thread_pool_runs_jobs() {
        let mut pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..1000 {
            let c = counter.clone();
            pool.submit(move || {
                c.fetch_add(1, Ordering::Relaxed);
            });
        }

        std::thread::sleep(Duration::from_millis(100));
        pool.shutdown();

        assert!(counter.load(Ordering::Relaxed) > 0);
    }
}
