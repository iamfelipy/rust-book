/*
  10.1 - Generic Data Types
*/

/*
  In Function Definitions
  Em Definições de Função
*/

// - exemplo: duas funções sem generics que duplicam código
// encontrando o maior valor em um slice

fn largest_i32(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn largest_char(list: &[char]) -> &char {
  let mut largest = &list[0];

  for item in list {
    // item e largest, implementam partialOrd e > chama ela, o char vale unicode e entao faz comparação
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest_i32(&number_list);
  println!("The largest number is {result}");

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest_char(&char_list);
  println!("The largest char is {result}");
}

// exemplo: criando generic e removendo duplicação
// não compila pois T não implementa PartialOrd
// Rust, por outro lado, exige que você declare antecipadamente as capacidades esperadas dos tipos genéricos. 

fn largest<T>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

// erro

$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
