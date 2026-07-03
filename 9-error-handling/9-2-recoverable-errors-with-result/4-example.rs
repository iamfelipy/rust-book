// --- Shortcuts for Panic on Error: unwrap and expect
// alternativa ao match verboso
// Result tem varios metodos shortcut

-----------------

//.unwrap()
// unwrap ou panic atalho

// sem unwrap
let greeting_file = match greeting_file_result {
  Ok(file) => file,
  // mostra a mensagem de erro padrão definida pela lib io::Error
  Err(error) => panic!("Problem opening the file: {error:?}"),
};

// com unwrap
fn main() {
  // retorna o valor ou aciona panic
  let greeting_file = File::open("hello.txt").unwrap();
}

-----------------

// .expect()
// igual ao expect mas consigo definir a mensagem de erro
// em grandes bases de codigo isso pode ajudar a achar o erro rapido

// src/main.ts
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").expect(
        "hello.txt should be included in this project"
    );
}