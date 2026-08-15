/*
  - 10.2 - traits - defining shared behavior
  - Características: Definindo o Comportamento Compartilhado
*/

/*
  introduction
*/

/*
exemplo:
- varias structs que armazenam tipos e quantidades diferentes de texto

struct 1: NewArticle
    - text: armazena uma noticia publicada em um local especifico

struct 2: SocialPost
    - text: armazena no maximo 280 caracters
    - tem metadados que indica se é uma nova publicação, republicação ou uma resposta a outra publicação

objetivo: criar uma lib/biblioteca agregadora de midia que pode exibir resumos de dados armazenados em cada struct.

pra isso é necessario ter um resumo de cada tipo
*/

// src/lib.rs
pub trait Summary {
  fn summarize(&self) -> String;
}

/*
  Implementing a Trait on a Type
*/

// exemplo 1:

// src/lib.rs
pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct SocialPost {
  pub username: String,
  // assumindo que o conteúdo da postagem já está limitado a 280 caracteres.
  pub content: String,
  pub reply: bool,
  pub repost: bool,
}

impl Summary for SocialPost {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}

// src/main.ts
use aggregator::{SocialPost, Summary};
// Em Rust, mesmo que SocialPost implemente Summary, você precisa trazer a trait para o escopo para chamar os métodos dela.

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}


// exemplo 2: regra de chaamda: mesmo metodo em ambas as traits

trait A { fn hello(&self); }
trait B { fn hello(&self); }
struct X;
impl A for X {
    fn hello(&self) { println!("A"); }
}
impl B for X {
    fn hello(&self) { println!("B"); }
}
let x = X;
A::hello(&x); // chama A
B::hello(&x); // chama B

// A implementação de uma trait para um tipo é global no programa inteiro. Então se duas existissem, seria um conflito sem solução, mesmo vindo de crates diferentes.