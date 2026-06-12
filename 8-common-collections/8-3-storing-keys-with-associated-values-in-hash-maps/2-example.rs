// Updating a Hash Map

// sobrescrever um valor

  use std::collections::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{scores:?}");
  // resulado 25

// adicionando uma chave e um valor somente se uma chave não estiver presente.
  use std::collections::HashMap;

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{scores:?}");
  // resulado {"Yellow": 50, "Blue": 10}


// atualizando um valor com base no valor antigo.
  // Contagem de ocorrências de palavras usando um mapa hash que armazena palavras e contagens.
  use std::collections::HashMap;

  let text = "hello world wonderful world";

  let mut map = HashMap::new();
  
  // for in em Rust só aceita tipos que implementam o trait Iterator.
  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }

  println!("{map:?}");
  // imprimi em ordem arbitraria