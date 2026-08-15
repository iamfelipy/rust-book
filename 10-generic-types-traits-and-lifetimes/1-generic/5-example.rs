/*
  10.1 - Generic Data Types
*/

/*
  In Method Definitions
  Em Definições de Método
*/

// examplo 1: - exemplo: Implementando um método nomeado “x” na Point<T>struct que retornará uma referência ao “x” campo do tipo T

// src/main.rs
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
      &self.x
  }
}

fn main() {
  let p = Point { x: 5, y: 10 };

  println!("p.x = {}", p.x());
}

// exemplo 2: Um impl bloco que se aplica somente a uma struct com um tipo concreto específico para o parâmetro de tipo genérico T

// src/main.rs
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

// exemplo 3: exemplo: Um método que usa tipos genéricos diferentes da definição de sua estrutura.
// src/main.rs
struct Point<X1, Y1> {
  x: X1,
  y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
  fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
      Point {
          x: self.x,
          y: other.y,
      }
  }
}

fn main() {
  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);

  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}