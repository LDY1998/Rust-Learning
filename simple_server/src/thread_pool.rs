use std::thread;
use std::sync::mpsc;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    size: usize,
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        ThreadPool {
            threads,
            size
        }
    }

}