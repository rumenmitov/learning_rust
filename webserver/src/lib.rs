use std::thread;
use std::sync::{Arc, Mutex,mpsc};

struct Worker {
    id :usize,
    job :Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id :usize, receiver :Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let job = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Job received by worker with id: {}", id); 
                        job();
                    },

                    Err(_) => {
                        println!("Worker with id: {id} stopping; shutting down...");
                        break;
                    }
                }


            });

        Worker {
            id,
            job: Some(job),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads :Vec<Worker>,
    sender :Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size :usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut threads :Vec<Worker> = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in  0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f :F)
        where 
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.threads {
            println!("Shutting down worker with id: {}", worker.id);

            if let Some(job) = worker.job.take() {
                job.join().unwrap();
            }
        }
    }
}
