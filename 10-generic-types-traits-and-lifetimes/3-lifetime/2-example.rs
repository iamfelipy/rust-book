/*
  10.3 - Validating References with Lifetimes
*/

/*
  Generic Lifetimes in Functions
*/

// exemplo:  Uma main função que chama a longest função para encontrar a maior de duas fatias de string.

//  src/main.rs

// versao 1
// nao compila

fn main() {
  let string1 = String::from("abcd");
  // isso cria um slice: ponteiro pra um literal string
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {result}");
}

fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() { x } else { y }
}

// erro
$ cargo run
 Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
--> src/main.rs:9:33
|
9 | fn longest(x: &str, y: &str) -> &str {
|               ----     ----     ^ expected named lifetime parameter
|
= help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
|
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
|           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error


/*
  Lifetime Annotations in Function Signatures
*/

//  o tempo de vida da referência retornada pela longest função é o mesmo que o menor dos tempos de vida dos valores referenciados pelos argumentos da função.
// Em outras palavras, o lifetime genérico 'a receberá o lifetime concreto que é igual ao menor dos lifetimes de x e y. 


//  src/main.rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

// exemplo que compila

fn main() {
  let string1 = String::from("long string is long");

  {
    // Neste exemplo, string1 é válida até o fim do escopo externo, string2 é válida até o fim do escopo interno, e result referencia algo que é válido até o fim do escopo interno.
      let string2 = String::from("xyz");
      let result = longest(string1.as_str(), string2.as_str());
      println!("The longest string is {result}");
  }
}

// exemplo que não compila
fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
      // para result ser válido para a println! declaração, string2 precisaria ser válido até o final do escopo externo. 
      let string2 = String::from("xyz");
      result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {result}");
}

$ cargo run
 Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
--> src/main.rs:6:44
|
5 |         let string2 = String::from("xyz");
|             ------- binding `string2` declared here
6 |         result = longest(string1.as_str(), string2.as_str());
|                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
|     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {result}");
|                                     -------- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
