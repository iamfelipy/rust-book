
// Generic Types, Traits, and Lifetimes
// Tipos Genéricos, Características e Tempos de Vida

/*
  - Removing Duplication by Extracting a Function
*/

// exemplo 1: programa que encontra o maior numero em uma lista

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let mut largest = &number_list[0];

  for number in &number_list {
      if number > largest {
          largest = number;
      }
  }

  println!("The largest number is {largest}");
}

// exemplo 2: encontrar o maior numero em duas listas diferentes

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let mut largest = &number_list[0];

  for number in &number_list {
      if number > largest {
          largest = number;
      }
  }

  println!("The largest number is {largest}");

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  let mut largest = &number_list[0];

  for number in &number_list {
      if number > largest {
          largest = number;
      }
  }

  println!("The largest number is {largest}");
}

// exemplo: expressando o conceito de encontrar o maior número em uma lista de forma abstrata.

fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {result}");

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  let result = largest(&number_list);
  println!("The largest number is {result}");
}

// - Em seguida, usaremos essas mesmas etapas com genéricos para reduzir a duplicação de código. Da mesma forma que o corpo da função pode operar em um tipo abstrato list em vez de valores específicos, os genéricos permitem que o código opere em tipos abstratos.
//- Por exemplo, imagine que temos duas funções: uma que encontra o maior item em um conjunto de i32 valores e outra que encontra o maior item em outro conjunto de char valores. Como eliminaríamos essa duplicação?