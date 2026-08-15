/*
  10.3 - Validating References with Lifetimes
*/

/*
  Lifetime Annotations in Method Definitions
*/

// exemplo 1

// não preciso declarar o tempo de vida no parametro de level por causa da primeira regra de elisão
// sou obrigado a colocar o tempo de vida apos o impl, porque a struct armazena uma referencia
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
      3
  }
}

// exemplo 2: aplicando a terceira regra de elisão

// Existem duas lifetimes de entrada, então o Rust aplica a primeira regra de elisão de lifetimes e atribui a &self e a announcement suas próprias lifetimes. Então, como um dos parâmetros é &self, o tipo de retorno recebe a lifetime de &self, e todas as lifetimes foram contabilizadas.
impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("Attention please: {announcement}");
      self.part
  }
}

// --- exemplos a mais gerados com ia

// exemplo 3: 
// Quando usamos announce_and_return_part, ele garante que o valor retornado (self.part) nunca vive mais do que a referência ao próprio struct (&self). Assim, este código funciona:

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().unwrap();
let excerpt = ImportantExcerpt { part: first_sentence };
let part = excerpt.announce_and_return_part("Here is the excerpt!");
println!("Returned: {part}");
// Se você tentasse retornar part após excerpt (ou novel) sair do escopo, daria erro, garantindo segurança de memória via lifetimes.

// Exemplo prático: vai dar erro ao tentar usar `part` após `excerpt` (ou `novel`) sair do escopo

fn main() {
    let part: &str;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let excerpt = ImportantExcerpt { part: first_sentence };
        part = excerpt.announce_and_return_part("Here is the excerpt!");
        println!("Dentro do escopo: {part}");
    }
    // Erro! Aqui tentaremos utilizar `part` após o dado do qual ele depende (novel) ter sido dropado
    println!("Fora do escopo: {part}");
    // Isso não compila:
    // error[E0597]: `novel` does not live long enough
    //    --> src/main.rs:XX:XX
    //     |
    //  XX |         part = excerpt.announce_and_return_part("Here is the excerpt!");
    //     |         ----                                                   ------- borrow later used here
    //     |         |
    //     |         borrowed value does not live long enough

    // Para ver o erro, descomente a linha acima
}