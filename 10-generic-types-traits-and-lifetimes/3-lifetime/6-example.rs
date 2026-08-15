/*
  10.3 - Validating References with Lifetimes
*/

/*
  Lifetime Elision
*/

// exemplo 1:  codigo que compila sem anotações de tempo de vida
// a equipe Rust descobriu que os programadores estavam inserindo as mesmas anotações de tempo de vida repetidamente em situações específicas. Essas situações eram previsíveis e seguiam alguns padrões determinísticos.
// Os padrões programados na análise de referências do Rust são chamados de regras de elisão de tempo de vida. 
// São um conjunto de casos particulares que o compilador considerará e, se seu código se encaixar nesses casos, você não precisa escrever os tempos de vida explicitamente.

// src/lib.rs
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}
