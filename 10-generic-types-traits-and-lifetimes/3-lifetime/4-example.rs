/*
  10.3 - Validating References with Lifetimes
*/

/*
  Thinking in Terms of Lifetimes
*/

// example 1: so preciso anotar se retornar mais de um lifetime

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
  x
}

// example 2: não posso retornar uma referencia criada dentro de uma função
// src: main.rs
// esse codigo nao compila
fn longest<'a>(x: &str, y: &str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
}

// erro
$ cargo run
 Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return value referencing local variable `result`
--> src/main.rs:11:5
 |
11 |     result.as_str()
 |     ------^^^^^^^^^
 |     |
 |     returns a value referencing data owned by the current function
 |     `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
