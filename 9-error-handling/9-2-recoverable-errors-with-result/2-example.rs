// -- Matching on Different Errors
// Correspondência com base em erros diferentes

// exemplo: executar ações diferentes para diferentes motivos de falha
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        // Dentro de Ok está o valor retornado em caso de sucesso — neste caso, uma instância de std::fs::File, que representa o arquivo aberto com sucesso.
        Ok(file) => file,
        // error = io:Error
        Err(error) =>
                  // retorna enum io:ErrorKind
            match error.kind() {
                ErrorKind::NotFound =>
                    match File::create("hello.txt") {
                                // return para greeting_file
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {e:?}"),
                    }
                _ => {
                    panic!("Problem opening the file: {error:?}");
                }
            }
    };
}

// Alternatives to Using match with Result<T, E>
// exemplo: versão anterior simplificada
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // example: closure, chapter 13
    //  simplificar match expressões aninhadas complexas quando você estiver lidando com erros/Result.
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}