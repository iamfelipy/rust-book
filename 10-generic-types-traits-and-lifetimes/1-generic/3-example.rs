/*
  10.1 - Generic Data Types
*/

/*
  In Struct Definitions
  Em Definições de Estrutura
*/

// exemplo 1: usando um paremetro generic em struct
// codigo nao compila, pois x e y devem ser do mesmo tipo

struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let integer = Point { x: 5, y: 4.0 };
  let float = Point { x: 1.0, y: 10 };
}

erro ao compilar:
$ cargo run
 Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
--> src/main.rs:7:38
|
7 |     let wont_work = Point { x: 5, y: 4.0 };
|                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error

// exemplo 2: correção

struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let both_integer = Point { x: 5, y: 10 };
  let both_float = Point { x: 1.0, y: 4.0 };
  let integer_and_float = Point { x: 5, y: 4.0 };
}