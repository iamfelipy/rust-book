// v1

use std::time::Duration;

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        // retorna future
        // não foi esperado com wait o spawn_task
        // O programa pode terminar antes do "first task" concluir, então nem todas as mensagens do "first task" serão impressas.
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}

hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!


// v2
use std::time::Duration;

fn main() {
    trpl::run(async {
                  // retorna uma future
      let handle = trpl::spawn_task(async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
      });
      
      for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
      
      handle.await.unwrap();
    });
}

hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!



// v3
use std::time::Duration;

fn main() {
  // aguardando as futures terminar e intercalando
  trpl::run(async {
    let fut1 = async {
      for i in 1..10 {
          println!("hi number {i} from the first task!");
          trpl::sleep(Duration::from_millis(500)).await;
      }
    };
    
    let fut2 = async {
      for i in 1..5 {
          println!("hi number {i} from the second task!");
          trpl::sleep(Duration::from_millis(500)).await;
      }
    };
    
    trpl::join(fut1, fut2).await;
  });
}

hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!

