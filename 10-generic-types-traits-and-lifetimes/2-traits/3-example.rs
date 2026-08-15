/*
  - 10.2 - traits - defining shared behavior
  - Características: Definindo o Comportamento Compartilhado
*/

/*
  Default Implementations
*/

// exemplo: Definindo uma Summary característica com uma implementação padrão do summarize método

// src/lib.rs
pub trait Summary {
  fn summarize(&self) -> String {
      String::from("(Read more...)")
  }
}

// especificamos um impl bloco vazio com 
// --snip
impl Summary for NewsArticle {}

//src/main.rs
// chamar o summarize método em uma instância de ` NewsArticle
let article = NewsArticle {
  headline: String::from("Penguins win the Stanley Cup Championship!"),
  location: String::from("Pittsburgh, PA, USA"),
  author: String::from("Iceburgh"),
  content: String::from(
      "The Pittsburgh Penguins once again are the best \
           hockey team in the NHL."
  ),
};

println!("New article available! {}", article.summarize());

// a sintaxe para sobrescrever uma implementação padrão é a mesma que a sintaxe para implementar um método de trait que não possui uma implementação padrão.


// exemplo: Implementações padrão podem chamar outros métodos na mesma trait, mesmo que esses outros métodos não tenham uma implementação padrão. 

// src/lib.rs
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
  }
}

impl Summary for SocialPost {
  fn summarize_author(&self) -> String {
      format!("@{}", self.username)
  }
}

// src/main.rs
let post = SocialPost {
  username: String::from("horse_ebooks"),
  content: String::from("of course, as you probably already know, people"),
  reply: false,
  repost: false,
};

println!("1 new post: {}", post.summarize());


// Note que não é possível chamar a implementação padrão a partir de uma implementação sobrescrita desse mesmo método.