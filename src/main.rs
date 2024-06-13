use std::thread;

use rust_pattern::channels::{
  actor::{self, Actor},
  fan_in_out::fan_execute,
  pipeline::pipline,
  pro_com::producer_consumer,
  pub_sub::{self, PubSub},
  worker_pool::WorkerPool,
};

fn main() {
  //   producer_consumer();
  //   pipline()

  //   let worker_pool = WorkerPool::new(5);

  //   for job in 0..8 {
  //     worker_pool.execute(job);
  //   }

  // let hub = PubSub::<&str>::new();
  // let (sub_id, rx) = hub.subcribe();
  // let handle = thread::spawn(move || {
  //   for received in rx {
  //     println!("Received: {}", received);
  //   }
  // });

  // hub.publish("Hello world");
  // hub.unsubcribe(sub_id);
  // handle.join().unwrap();

  // fan_execute();

  let actor = Actor::new("actor 1".to_string(), true);
  let actor2 = Actor::new("actor 2".to_string(), false);

  actor.start();
  actor2.start();

  actor.send("Hello from main to actor 1".to_string());
  actor2.send("Hello from main to actor 2".to_string());

  actor.stop();
  actor2.stop();
}
