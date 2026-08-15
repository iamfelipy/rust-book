/*
  quiz 3
*/Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

struct Point<T> { x: T, y: T }
impl Point<i32> {
  fn f(&self) -> &i32 { &self.y }
}
impl<T> Point<T> {
  fn f(&self) -> &T { &self.x }
}
fn main() {
  let p: Point<i32> = Point { x: 1, y: 2 };
  println!("{}", p.f());
}

resposta: 
This program does not compile.

why?
Essas definições de f entram em conflito, e não há como o Rust determinar qual f deve ser usado quando p.f() é chamado. Portanto, trata-se de um erro de compilação.