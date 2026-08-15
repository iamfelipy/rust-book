// quiz 1
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

use std::fmt::Display;
fn displayable<T: Display>(t: T) -> impl Display { t }
fn main() {
  let s = String::from("hello");
  let mut s2 = displayable(s);
  s2.push_str(" world");
  println!("{s2}");
}

answer
This program does not compile.

context
Como displayable retorna impl Display, então só sabemos que s2 é algum tipo que implementa Display, não que seja uma String que tenha o método push_str. Portanto não podemos chamar s2.push_str(...). Se o tipo de retorno de displayable fosse -> T, então este programa compilaria.

// quiz 2
What is the smallest set of trait bounds on T needed to make this function type-check?
Qual é o menor conjunto de restrições de traço em T necessário para fazer essa função compilar (type-check)?


fn f<T: /* ??? */>(t: &T) {
  let t2 = t.clone();
  println!("{t2}");
}

answer
Clone + Display

context
Porque clone é chamado e os colchetes {} de formatação são usados, então T deve implementar Clone e Display.