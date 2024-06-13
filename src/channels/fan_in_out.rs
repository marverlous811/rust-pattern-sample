use std::{sync::mpsc, thread};

pub fn fan_execute() {
  let (tx, rx) = mpsc::channel::<i32>();
  // Fan-in: Distributing work to multiple workers
  for i in 0..5 {
    let tx_clone = tx.clone();
    thread::spawn(move || {
      tx_clone.send(i * 2).unwrap();
    });
  }

  drop(tx);
  //Fan-out: Collecting results for multiple workers
  for received in rx {
    println!("Got: {received}");
  }
}
