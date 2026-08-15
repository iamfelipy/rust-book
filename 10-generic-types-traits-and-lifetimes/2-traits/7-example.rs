/*
  - 10.2 - traits - defining shared behavior
  - Características: Definindo o Comportamento Compartilhado
*/

/*
  Using Trait Bounds to Conditionally Implement Methods
  Utilizando limites de características para implementar métodos condicionalmente
*/

//caso 1: criando metodo restrito a tipo
// só consegue usar o método cmp_display se o tipo T implementar as traits Display e PartialOrd, pois o método está restrito por esses bounds.

// src/lib.rs
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// caso 2: Também podemos implementar condicionalmente uma trait para qualquer tipo que implemente outra trait.
// blanket implementations
// 

impl<T: Display> ToString for T {
  // --snip--
}
// ToString é uma trait da biblioteca padrão do Rust. Ela define o método to_string, que converte um valor em String. Qualquer tipo que implemente Display já implementa automaticamente ToString devido a essa implementação genérica (impl<T: Display> ToString for T).
let s = 3.to_string();

