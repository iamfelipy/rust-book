//quiz 1
Pergunta 1
Qual das seguintes afirmações descreve melhor a função da palavra-chave use?

Você respondeu:
use reduz a verbosidade ao se referir a itens no caminho utilizado

Contexto: use permite referências a itens no caminho utilizado que são mais curtas do que sem o use, reduzindo a verbosidade da referência.

//quiz 2
Questão 2
Considere este módulo e declaração use:

pub mod parent {
  pub fn a() {}
  fn b() {}
  pub mod child {
    pub fn c() {}
  }
}
fn main() {
  use parent::{*, child as alias};
  // ...
}

Dentro de main, qual é o número total de caminhos que podem se referir a a, b ou c (não incluindo aqueles que usam self, super ou crate)? Escreva sua resposta como um dígito, como 0 ou 1. Por exemplo, se os únicos dois caminhos válidos fossem a e parent::b, então a resposta seria 2.

resposta correta: 5
Contexto: 
Existem dois caminhos para a: 
parent::a e a. 
Não há caminhos para b, porque é privado. 
Há três caminhos para c: parent::child::c, child::c, alias::c.