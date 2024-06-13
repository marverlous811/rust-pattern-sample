use std::{
  sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
  },
  thread,
};

pub struct Actor {
  sender: Sender<Option<String>>,
  receiver: Mutex<Option<Receiver<Option<String>>>>,
  name: String,
  quiet: bool,
  handle: Mutex<Option<thread::JoinHandle<()>>>,
}

impl Actor {
  pub fn new(name: String, quiet: bool) -> Arc<Self> {
    let (tx, rx) = mpsc::channel::<Option<String>>();
    Self {
      sender: tx,
      receiver: Mutex::new(Some(rx)),
      name,
      quiet,
      handle: Mutex::new(None),
    }
    .into()
  }

  pub fn start(self: &Arc<Self>) {
    let receiver = self.receiver.lock().unwrap().take().unwrap();
    let me = self.clone();
    let handle = thread::spawn(move || {
      for msg in receiver {
        if let Some(msg) = msg {
          if !me.quiet {
            println!("{}: {}", me.name, msg);
          }
        } else {
          break;
        }
      }
      println!("{}: shutting down", me.name);
    });
    self.handle.lock().unwrap().replace(handle);
  }

  pub fn send(&self, msg: String) {
    self.sender.send(Some(msg)).unwrap();
  }

  pub fn stop(&self) {
    if let Some(handle) = self.handle.lock().unwrap().take() {
      self.sender.send(None).unwrap();
      handle.join().unwrap();
    }
  }
}
