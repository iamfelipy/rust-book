Questão 1
Dada uma expressão arbitrária e do tipo Result<T, E>, qual trecho de código melhor representa como e ? é traduzido?

resposta:
match e {
  Ok(x) => x,
  Err(err) => { return Err(err); }
}

Contexto: Se e é um Result, então e? extrai o valor dentro do Ok, se possível; caso contrário, retorna o Err a partir da função atual.

--------------------------------

questão 2
Determine se o programa passará pelo compilador. Se passar, escreva a saída esperada do programa caso fosse executado.

// suponha que hello.txt contém "will"
fn read_username_from_file() -> Option<String> {
  let mut username_file = File::open("hello.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username)?;
  Some(username)
}
fn main() {
  println!("{}", read_username_from_file().unwrap()); 
}

A resposta correta é:
Este programa não compila.

Contexto: File::open retorna um Result, mas o tipo de retorno de read_username_from_file espera um Option. Portanto é inválido usar o operador ? até que o Result tenha sido convertido em Option (por exemplo com o método Result::ok).