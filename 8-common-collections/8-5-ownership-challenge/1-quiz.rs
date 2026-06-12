// Program 1:
//
/// Removes all the zeros in-place from a vector of integers.
fn remove_zeros(v: &mut Vec<i32>) {
  for (i, t) in v.iter().enumerate().rev() {
      if *t == 0 {
          v.remove(i);
          // Chama um método que reduz a capacidade alocada do vetor para caber exatamente o número atual de elementos, liberando memória não utilizada. Não altera os elementos.
          v.shrink_to_fit();
      }
  }
}

// pergunta 1
Se você tentar compilar esta função, qual das opções a seguir melhor descreve o erro do compilador que você receberia?

//resposta
answered: v.remove(i) cannot borrow v as mutable

context: A função v.iter() empresta imutavelmente o vetor v durante a duração do laço for. No entanto, v.remove(i) requer uma referência mutável para v. Portanto, v.remove(i) não pode emprestar v como mutável, pois isso conflitaria com o iterador.

------------------------------------------

// pergunta 2
/*
Normally if you try to compile this function, the compiler returns the following error:

error[E0502]: cannot borrow `*v` as mutable because it is also borrowed as immutable
  --> test.rs:5:13
   |
4  |     for (i, t) in v.iter().enumerate().rev() {
   |                  -- immutable borrow occurs here
                      -- immutable borrow later used here
5  |         if *t == 0 {
6  |             v.remove(i);
   |             ^^^^^^^^^^^ mutable borrow occurs here
7  |         }

-----------------

Suponha que o compilador NÃO rejeitou esta função. Quais (se houver) dos programas a seguir (1) passariam no compilador e (2) possivelmente causariam comportamento indefinido se executados? Marque cada programa que satisfaça ambos os critérios, OU marque "Nenhum destes programas" se nenhum satisfizer.
*/

// resposta: 
// ---------------------------------------
//   let mut v = vec![5, 5, 0];
//   remove_zeros(&mut v);
//   println!("{:?}", v);
// ---------------------------------------
//   let mut v = vec![1, 2, 0, 3];
//   remove_zeros(&mut v);

// Para violar a segurança de memória, remove_zeros deve ser chamado com um vetor que contenha um zero depois do primeiro elemento. A chamada a v.shrink_to_fit() desalocará a memória pertencente ao vetor (devido ao redimensionamento), o que invalidará o iterador v.iter() que contém um ponteiro para os dados antigos. Observe que ler o vetor v após chamar remove_zeros não é essencial para a violação de segurança, já que o problema é interno a remove_zeros.

------------------------------------------
pergunta 3:

correção que atende melhor a estes três critérios:

1-A função corrigida passa no compilador Rust,
2-A função corrigida preserva a intenção do código original, e
3-A função corrigida não introduz ineficiências desnecessárias

// antes
fn remove_zeros(v: &mut Vec<i32>) {
  for (i, t) in v.iter().enumerate().rev() {
      if *t == 0 {
          v.remove(i);
          v.shrink_to_fit();
      }
  }
}

// resposta
fn remove_zeros(v: &mut Vec<i32>) {
    for i in (0..v.len()).rev() {
        if v[i] == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}

por que?
Contexto: Qualquer estratégia que exija alocar um novo vetor, seja via Vec::clone ou Vec::new, requer alocação adicional desnecessária. Portanto, a estratégia mais simples que funciona é iterar apenas sobre os índices 0 .. v.len(), o que não toma emprestado v. Fazemos isso em ordem reversa para evitar remover índices inexistentes.

Como no Problema 1, a estratégia mais idiomática é na verdade usar uma função embutida que não discutimos, Vec::retain. Essa função mantém apenas os elementos de um vetor que passam em um predicado, mas faz isso com maior eficiência de memória.