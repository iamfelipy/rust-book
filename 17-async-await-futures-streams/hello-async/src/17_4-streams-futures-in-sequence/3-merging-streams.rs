//v1 - sem tratamento de erros e com unwrap
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn get_messages() -> impl Stream<Item = String> {
  let (tx, rx) = trpl::channel();

  trpl::spawn_task(async move {
      let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
      for (index, message) in messages.into_iter().enumerate() {
          let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
          trpl::sleep(Duration::from_millis(time_to_sleep)).await;

          tx.send(format!("Message: '{message}'")).unwrap();
      }
  });

  ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
  let (tx, rx) = trpl::channel();

  trpl::spawn_task(async move {
      let mut count = 0;
      loop {
          trpl::sleep(Duration::from_millis(1)).await;
          count += 1;
          tx.send(count).unwrap();
      }
  });

  ReceiverStream::new(rx)
}

fn main() {
    trpl::run(async {
      let messages = get_messages().timeout(Duration::from_millis(200));
      let intervals = get_intervals()
          .map(|count| format!("Interval: {count}"))
          .throttle(Duration::from_millis(100))
          .timeout(Duration::from_secs(10));
      let merged = messages.merge(intervals).take(20);
      let mut stream = pin!(merged);
    })
}


//v2 - tratando erros - canal com erro
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

fn main() {
    trpl::run(async {
        // timeout = limite de tempo, timeout retorna um result
				let messages = get_messages().timeout(Duration::from_millis(200));
				let intervals = get_intervals()
				    .map(|count| format!("Interval: {count}"))
				    .throttle(Duration::from_millis(100))
				    .timeout(Duration::from_secs(10));
				let merged = messages.merge(intervals).take(20);
				let mut stream = pin!(merged);


        while let Some(result) = messages.next().await {
            match result {
                // ok: indica que a mensagem chegou a tempo
                Ok(message) => println!("{message}"),
                // err: indica que o tempo limite expirou antes que qualquer mensagem chegasse
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
