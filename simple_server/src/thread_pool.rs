use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::fmt::Display;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id: id,
            thread: Some(thread::spawn(move || {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("The worker: {} got job, executing", id);

                job();
            })),
        }
    }
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::from(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static ,
    {

        let job = Box::new(f);
        self.sender.send(job).unwrap();

    }

}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending terminating message to all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}