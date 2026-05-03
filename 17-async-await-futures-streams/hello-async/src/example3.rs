// v1

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
      let (tx, mut rx) = trpl::channel();

      let val = String::from("hi");
      tx.send(val).unwrap();

      let received = rx.recv().await.unwrap();
      println!("Got: {received}");
    });
}

// v2
// Observe dois detalhes neste exemplo. Primeiro, a mensagem chegará imediatamente. Segundo, embora usemos um future aqui, ainda não há concorrência. Tudo na listagem acontece em sequência, exatamente como aconteceria se não houvesse nenhum future envolvido.

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future")
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
        
        // versão if let só que para loop
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

// v3

fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;

    });
}

// v4
fn main() {
    // precisa usar run pra main suportar async
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        
        // uma das formas de fechar o canal é tx sendo descartado
        // quando bloco async termina ele descarta as variaeis
        // move mude tx de referencia para onwership
        // então ele é descartado
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
        
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        
        trpl::join(tx_fut, rx_fut).await;
    });
}

//v5
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

// 
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'

