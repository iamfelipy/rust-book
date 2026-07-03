// --- Where the ? Operator Can Be Used

// só podemos usar o ? operador em uma função que retorna Result, Option, ou outro tipo que implementa FromResidual.

/*
  example: main

  esse codigo da erro, a main retorna ()
*/

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}

/*
  exemplo: usando ? com Option

  uma função que encontra o último caractere da primeira linha do texto fornecido.

  se for some continua, se for none retorna para função acima
*/

fn last_char_of_first_line(text: &str) -> Option<char> {

  text.lines() // return iterator
    .next()? // pego a primeira linha, return Option, ? return Some -> slice
    .chars() // return iterator
    .last() // return Option
}

/*
  exemplo: convertendo entre Result e Option
*/

// x.ok()?, Result → Option
fn h() -> Option<i32> {
  let x: Result<i32, String> = Ok(10);
  let y = x.ok()?; // agora é Option
  Some(y)
}

// x.ok_or?, Option → Result
fn k() -> Result<i32, String> {
  let x: Option<i32> = Some(10);
  let y = x.ok_or("erro")?; // agora é Result
  Ok(y)
}

/*
  exemplo: Alterar main para retornar Result<(), E>permite o uso do ?operador em Result valores.

  dyn aparece no chapter 18
  box aparece no capitulo 15

  convencao que vem do c, retornar inteiro 0 para sucesso, e diferente de 0 para erro
  e oque acontece ao usar result na main
  - o tipo `Result` implementa a `Termination` trait. Isso permite que `main` retorne `Result` e o Rust trate o valor de retorno automaticamente:
    - Se `Ok(_)`, retorna código de saída 0.
    - Se `Err(_)`, imprime o erro e retorna código de saída diferente de zero.
*/

// Error é uma trait da biblioteca padrão, não uma struct. Isso permite definir erros customizados implementando essa trait.
use std::error::Error;
use std::fs::File;
                        // suporta qualquer tipo de erro que implemente a trait std::error:Error
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}