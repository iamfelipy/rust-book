//quiz 1
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.


fn main() {
  let mut v = vec![1, 2, 3];
  for i in &mut v {
    v.push(*i);
  }
  println!("{} {} {}", v[3], v[4], v[5]);
}
answer: This program does not compile.
context: Mesmo que v seja emprestado mutavelmente, isso só permite que i seja mutado dentro do laço for, não v. Portanto chamar v.push é um erro de propriedade.

//quiz 2
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

fn main() {
  let mut v: Vec<i32> = vec![1, 2, 3];
  let mut v2: Vec<&mut i32> = Vec::new();
  // Sim, são diferentes: &mut v cria uma referência mutável ao vetor, enquanto v.iter_mut() cria um iterador sobre referências mutáveis aos elementos. Apenas no contexto do for o compilador transforma for i in &mut v automaticamente em uma iteração mutável.
  // i não é o valor i32 mas sim uma referência mutável para o valor i32, pois o iterator guarda uma referência mutável para o valor i32.
  for i in &mut v {
    v2.push(i);
  }
  *v2[0] = 5;
  let a = *v2[0];
  let b = v[0];
  println!("{a} {b}");
}
answer: 
The output of this program will be: 5 5 
context: i tem o tipo &mut i32, o que significa que é um ponteiro para um número dentro de v. Então, se colocarmos i em v2, v2 passa a conter ponteiros para v. Portanto, mutar v2[0] na verdade muta v[0].