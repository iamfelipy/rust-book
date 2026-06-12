// quiz 1
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

use std::collections::HashMap;
fn main() {
  let mut h = HashMap::new();
  h.insert("k1", 0);
  let v1 = &h["k1"];
  h.insert("k2", 1);
  let v2 = &h["k2"];
  println!("{} {}", v1, v2);
}
// resposta: não compila
// por que: 
h não pode ser mutado (h.insert("k2", 1)) enquanto uma referência imutável (v1) a ele estiver viva.

// quiz 2
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

use std::collections::HashMap;
fn main() {
  // usize é um tipo inteiro sem sinal usado para indexação e tamanho, cujo tamanho depende da arquitetura (32 ou 64 bits).
  let mut h: HashMap<char, Vec<usize>> = HashMap::new();
  // chars() retorna um iterador, e é o iterador que possui o método enumerate(), não o tipo char.
  for (i, c) in "hello!".chars().enumerate() {
    h.entry(c).or_insert(Vec::new()).push(i);
  }
  let mut sum = 0;
  for i in h.get(&'l').unwrap() {
    sum += *i;
  }
  println!("{}", sum);
}
// resultado: 5
// por que: 
Este programa armazena um vetor de índices para cada ocorrência de uma determinada letra em um hashmap. Em seguida, soma todos os índices da letra 'l', que ocorrem nos índices 2 e 3 na string "hello!".
