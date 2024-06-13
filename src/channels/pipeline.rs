use std::thread;

pub fn pipline() {
  thread::scope(|scope| {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();
    let (tx2, rx2) = std::sync::mpsc::channel::<i32>();

    scope.spawn(move || {
      let data = vec![1, 2, 3, 4, 5];
      for x in data {
        tx.send(x * x).unwrap()
      }
    });

    scope.spawn(move || {
      for recv in rx {
        tx2.send(recv + 1).unwrap();
      }
    });

    scope.spawn(move || {
      for recv in rx2 {
        println!("Received {}", recv);
      }
    });
  });
}
