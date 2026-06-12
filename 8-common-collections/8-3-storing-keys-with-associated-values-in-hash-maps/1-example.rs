// hashmap
// HashMap<K, V>
// Por exemplo, em um jogo, você poderia registrar a pontuação de cada equipe em um mapa de hash onde cada chave é o nome de uma equipe e os valores são a pontuação de cada equipe. Dado o nome de uma equipe, você pode recuperar sua pontuação.


// Creating a New Hash Map

  use std::collections::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);    


// Accessing Values in a Hash Map

  // como obter um valor do hash?
    let team_name = String::from("Blue");

    let score = scores
      // o get retorna um Option<&V>
      .get(&team_name)
      // copied retorna um Option<V>
      .copied()
      // unwrap_or retorna 0 se option não for some
      .unwrap_or(0);
  
  // Como iterar sobre um hash map?
    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // Este código imprimirá cada par em uma ordem arbitrária:
    // Ordem arbitrária = sem ordem fixa ou garantida.
    Yellow: 50
    Blue: 10

  
// Hash Maps and Ownership

  use std::collections::HashMap;

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // move, onwership
  // field_name and field_value are invalid at this point, try using them and
  // see what compiler error you get!
