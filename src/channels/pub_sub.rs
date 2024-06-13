use std::{
  collections::BTreeMap,
  sync::{
    atomic::AtomicUsize,
    mpsc::{self, SyncSender},
    Arc, Mutex,
  },
};

type SubcribeId = usize;

pub struct PubSub<T> {
  subcribers: Arc<Mutex<BTreeMap<SubcribeId, SyncSender<T>>>>,
  next_id: AtomicUsize,
}

impl<T: Send + 'static> PubSub<T> {
  pub fn new() -> Self {
    Self {
      subcribers: Arc::new(Mutex::new(BTreeMap::new())),
      next_id: AtomicUsize::new(0),
    }
  }

  pub fn subcribe(&self) -> (SubcribeId, mpsc::Receiver<T>) {
    let (tx, rx) = mpsc::sync_channel::<T>(512);
    let mut subcribers = self.subcribers.lock().unwrap();
    let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    subcribers.insert(id, tx);
    (id, rx)
  }

  pub fn unsubcribe(&self, id: SubcribeId) {
    let mut subcribers = self.subcribers.lock().unwrap();
    subcribers.remove(&id);
  }

  pub fn publish_deadlocking(&self, msg: T)
  where
    T: Clone,
  {
    let subcribers = self.subcribers.lock().unwrap();
    for tx in subcribers.values() {
      tx.send(msg.clone()).unwrap();
    }
  }

  pub fn publish(&self, msg: T)
  where
    T: Clone,
  {
    let subcribers = self.subcribers.lock().unwrap().clone();
    for tx in subcribers.values() {
      let _ = tx.send(msg.clone());
    }
  }
}
