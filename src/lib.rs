use std::borrow::Cow::Borrowed;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static> ;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,

}

impl ThreadPool {
    ///Create a new ThreadPool
    ///
    /// the size is the number of threads in the pool
    ///
    /// #Panics
    ///
    /// the `new` fn will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let new_worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(new_worker)
        }
        ThreadPool {workers, sender}
    }
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move|| loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job: executing...");
                job();
            })
        }
    }
}