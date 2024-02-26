use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// The ThreadPool holds a collection of threads and a channel to send jobs to the threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/// A Job is a trait object that holds a closure that can be sent to the worker threads.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// The Message enum is used to send messages to the worker threads.
enum Message {
    NewJob(Job),
    Terminate,
}

/// The ThreadPool implementation
impl ThreadPool {
    /// Create a new ThreadPool with a given size.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    /// Execute a closure on the worker threads.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/// The ThreadPool implementation for the Drop trait.
impl Drop for ThreadPool {
    /// Drop the ThreadPool and send a terminate message to all worker threads.
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                let thread = thread.join();
                match thread {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
        }
    }
}

/// The Worker struct holds the id of the worker and the thread.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new worker with a given id and a receiver.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
