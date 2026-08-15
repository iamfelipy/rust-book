/*
  10.3 - Validating References with Lifetimes
*/

/*
  Lifetime Annotations in Struct Definitions
*/

// exemplo 1: armazenando referencia em struct e adicionando notação de tempo de vida na referencia
// esse codigo compila, por que novel so sai do escopo depois de ImportantExcerpt

struct ImportantExcerpt<'a> {
  // slice de string
  part: &'a str,
}

fn main() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().unwrap();
  let i = ImportantExcerpt {
      part: first_sentence,
  };
}
