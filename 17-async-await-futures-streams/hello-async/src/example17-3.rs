// 17 - Working with Any Number of Futures

//v1
fn main() {
  // precisa usar run pra main suportar async
  trpl::run(async {
      let (tx, mut rx) = trpl::channel();

      let tx1 = tx.clone();
      let tx1_fut = async move {
          let vals = vec![
              String::from("hi"),
              String::from("from"),
              String::from("the"),
              String::from("future")
          ];

          for val in vals {
              tx1.send(val).unwrap();
              trpl::sleep(Duration::from_millis(500)).await;
          }
      };

      let rx_fut = async {
          while let Some(value) = rx.recv().await {
              println!("received '{value}'");
          }
      };

      let tx_fut = async move {
          let vals = vec![
              String::from("more"),
              String::from("messages"),
              String::from("for"),
              String::from("you")
          ];

          for val in vals {
              tx.send(val).unwrap();
              trpl::sleep(Duration::from_millis(1500)).await;
          }
      };

      // função especifica de trpl para suportar 3 futures
      trpl::join3(tx1_fut, tx_fut, rx_fut).await;
  });
}

//v2
use std::pin::Pin;

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
  
        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future")
            ];
  
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
  
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
  
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you")
            ];
  
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
  
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        
        trpl::join_all(futures).await;
    });
  }

//v3
use std::pin::Pin;

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
  
        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future")
            ];
  
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
  
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
  
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you")
            ];
  
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
  
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
    });
  }


  //v4
  use std::pin::Pin;

  fn main() {
      // precisa usar run pra main suportar async
      trpl::run(async {
          let a = async { 1u32 };
          let b = async { "Hello!" };
          let c = async { true };
          
          let (a_result, b_result, c_result) = trpl::join!(a, b, c);
          println!("{a_result}, {b_result}, {c_result}");
      });
}

//racing futures
//v1

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };
        
        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };
        
        trpl::race(slow, fast).await;
        
        /*
        output stdin
        "'slow' started"
        "'fast' started"
        "'fast' finished"
        "'slow' started"
        */
    });
}

/*
    YIELDING CONTROL TO THE RUNTIME
*/

//v1
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };
        
        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };
        
        trpl::race(a, b).await;
    });
}
/*
output:
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
*/

//v2
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let one_ms = Duration::from_millis(1);
        
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };
        
        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };
        trpl::race(a, b).await;
    });
}

/*
adicionamos trpl::sleep chamadas com pontos de espera entre cada chamada para slow. Agora o trabalho dos dois futuros está intercalad
output:
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.

*/

//v3
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let one_ms = Duration::from_millis(1);
        
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };
        
        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };
        trpl::race(a, b).await;
    });
}

//v4
fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        (async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }).await;
        let time = Instant::now() - start;
        println!("'sleep' version finished after {} seconds.", time.as_secs_f32());

        let start = Instant::now();
        (async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }).await;
        let time = Instant::now() - start;
        println!("'yield' version finished after {} seconds.", time.as_secs_f32());
    });
}

/*
    BUILDING OUR OWN ASYNC ABSTRACTIONS
*/



    
