// quiz 1

Qual chamada da função find_until causará um pânico em tempo de execução?

fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
  for i in 0 .. til {
    if v[i] == n {
      return Some(i);
    }
  }
  return None;
}

resposta:
find_until(&vec![1, 2, 3], 4, 4)

por que?
Contexto: Se til = 4, então para um vetor de comprimento 3, o laço for tentará indexar o vetor com i = 3, o que está fora dos limites. Essa função não entra em panic se n = 1 porque retorna antes de alcançar o índice fora dos limites.

// quiz 2
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

fn main() {
  let mut v = Vec::new();
  let s = String::from("Hello ");
  v.push(s);
  // se houver reallocação ao chamar push_str, s pode passar a apontar para memória inválida.
  v[0].push_str("world");
  println!("original: {}", s);
  println!("new: {}", v[0]);
}

resposta:
this program does not compile

Contexto: Vec::push move seu argumento, então s não pode ser usado após chamar v.push(s). Portanto, chamar println!("original: {}", s) não é seguro em termos de propriedade.

//quiz 3
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.


fn main() {
  let v = vec![String::from("Hello ")];
  // Tipos não copiáveis não podem ser movidos para fora de um vetor por indexação. Apenas métodos como Vec::remove permitem mover elementos para fora de um vetor.
  let mut s = v[0];
  s.push_str("world");
  println!("{s}");
}

resposta:
this program does not compile

Contexto: Tipos não copiáveis não podem ser movidos para fora de um vetor por indexação. Apenas métodos como Vec::remove permitem mover elementos para fora de um vetor. Veja a Seção 4.3 "Copiando vs. Movendo para Fora de uma Coleção" para uma explicação mais aprofundada do porquê.
não posso mover por que isso deixa um buraco
