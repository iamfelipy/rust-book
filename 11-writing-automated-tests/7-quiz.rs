// quiz 1
Qual é a anotação que você adiciona a uma função para indicar que ela é um teste de unidade?

Você respondeu:
#[test]

Contexto: Isso informa ao mecanismo de testes do Cargo para tratar a função como um teste e não como código de biblioteca.

// quiz 2
Suponha que você tenha uma função com a assinatura de tipo:

fn f(x: usize) -> Result<usize, String>;
E você quer testar que f(0) deve retornar Err(_). 
Qual das seguintes NÃO é uma forma válida de testar isso?

A resposta correta é:
#[test]
#[should_err]
fn test() -> Result<usize, String> {
f(0)
}

Contexto: should_err não existe em Rust — qualquer teste que retorne um Result deve retornar Ok para passar.