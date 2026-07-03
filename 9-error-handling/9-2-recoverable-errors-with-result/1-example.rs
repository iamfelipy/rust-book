// -- introduction

// Result
enum Result<T, E> {
  Ok(T),
  Err(E),
}


/*
  - com a result a função consegue me dizer se deu certo ou falhou
- file::open retorna result<T,E>
- file::open prenche o tipo T(sucesso) e E(error) automaticamente
- T = std::fs::File
    - which is a file handle
      - que é um identificador de arquivo
      - Significa que representa uma referência ao arquivo aberto, permitindo ler ou escrever nele.
    - instancia de FileHandle, contem identificador do arquivo
- E = std::io::Error
    - sera uma instancia de Err com informações do erro
*/

// example : lendo arquivo
// exemplo: usando Match para lidar com as variantes Result
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}

// ao executar vai dar mostrar a mensagem do panic