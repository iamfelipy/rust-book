/*
  quiz 2
*/

Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

fn print_slice<T>(v: &[T]) {
  for x in v {
    println!("{x}");
  }
}
fn main() {
  print_slice(&[1, 2, 3]);
}

answer:
This program does not compile.

Contexto: Se um tipo é genérico (como T), não podemos assumir nada sobre ele, incluindo a capacidade de transformá-lo em uma string. Portanto, println!("{x}") é inválido porque x: &T.

- para funcionar com println precis da traif Display, e como T pode não ter, não funciona

bound = restrição que coloco em T 