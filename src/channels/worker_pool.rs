use std::{
  mem,
  sync::{mpsc, Arc, Mutex},
  thread,
};

struct Worker {
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Option<i32>>>>) -> Self {
    let thread = thread::spawn(move || loop {
      if let Some(job) = receiver.lock().unwrap().recv().unwrap() {
        println!("worker {id} got a job {job}");
      } else {
        println!("worker {id} will be free");
        break;
      }
    });
    Self { thread }
  }
}

pub struct WorkerPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Option<i32>>,
}

impl WorkerPool {
  pub fn new(size: usize) -> Self {
    let (tx, rx) = mpsc::channel::<Option<i32>>();
    let receiver = Arc::new(Mutex::new(rx));

    let workers: Vec<Worker> = (0..size).map(|id| Worker::new(id, receiver.clone())).collect();

    Self { workers, sender: tx }
  }

  pub fn execute(&self, job: i32) {
    self.sender.send(Some(job)).unwrap();
  }
}

impl Drop for WorkerPool {
  fn drop(&mut self) {
    for _ in &self.workers {
      self.sender.send(None).unwrap();
    }

    for worker in mem::take(&mut self.workers) {
      worker.thread.join().unwrap();
    }
  }
}
