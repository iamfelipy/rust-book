/*
  - 10.2 - traits - defining shared behavior
  - Características: Definindo o Comportamento Compartilhado
*/

/*
  Returning Types That Implement Traits
  Retornando tipos que implementam características
*/

// exemplo 1:

// não nomeio o tipo concreto retornado
// muito util em iterators e closures

fn returns_summarizable() -> impl Summary {
  SocialPost {
      username: String::from("horse_ebooks"),
      content: String::from(
          "of course, as you probably already know, people",
      ),
      reply: false,
      repost: false,
  }
}

// exemplo 2:
// regra: eu só posso usar impl Trait se estiver retornando um unico tipo

// esse codigo não compila, é retornando dois tipo diferentes
// isso não é permitido devido a restrições sobre como a impl Trait sintaxe é implementada no compilador
// a soluçao pra isso esta no capitulo 18: “Usando Objetos de Trait que Permitem Valores de Tipos Diferentes” do Capítulo 18.
fn returns_summarizable(switch: bool) -> impl Summary {
  if switch {
      NewsArticle {
          headline: String::from(
              "Penguins win the Stanley Cup Championship!",
          ),
          location: String::from("Pittsburgh, PA, USA"),
          author: String::from("Iceburgh"),
          content: String::from(
              "The Pittsburgh Penguins once again are the best \
               hockey team in the NHL.",
          ),
      }
  } else {
      SocialPost {
          username: String::from("horse_ebooks"),
          content: String::from(
              "of course, as you probably already know, people",
          ),
          reply: false,
          repost: false,
      }
  }
}