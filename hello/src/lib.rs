#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::thread;
use std::sync::{mpsc, Mutex, Arc};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, recevier) = mpsc::channel();
        let recevier = Arc::new(Mutex::new(recevier));
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&recevier)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F) 
        where F: FnOnce() + Send + 'static {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

impl Drop for ThreadPool {
   fn drop(&mut self) {
       println!("Sending terminate message to all workers.");

       for _ in &mut self.workers {
           self.sender.send(Message::Terminate).unwrap();
       }

       println!("Shutting down all workers.");
       
       for worker in &mut self.workers {
           println!("Shutting down worker {}", worker.id);

           if let Some(thread) = worker.thread.take() {
               thread.join().unwrap();
           }
       }
   }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a jog; executing.", id);
                        job()
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}