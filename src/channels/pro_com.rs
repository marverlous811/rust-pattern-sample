use std::{thread, time::Duration};

pub fn producer_consumer() {
  let (tx, rx) = std::sync::mpsc::channel::<i32>();

  let producer = thread::spawn(move || {
    let data = vec![1, 2, 3, 4, 5];
    data.iter().for_each(|x| match tx.send(*x) {
      Ok(_) => {
        println!("Sent {}", x);
        thread::sleep(Duration::from_millis(500));
      }
      Err(e) => {
        println!("Error when sent {:?}", e);
      }
    });
  });

  let consumer = thread::spawn(move || {
    for receive in rx {
      println!("Received {}", receive);
    }
  });

  producer.join().unwrap();
  consumer.join().unwrap();
}

#[cfg(test)]
mod test {
  #[test]
  fn test_producer_consumer() {
    super::producer_consumer();
  }
}
