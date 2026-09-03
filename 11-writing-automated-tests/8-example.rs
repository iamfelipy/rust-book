/*
  - Controlling How Tests Are Run
*/


/*
  cargo test
      - por padrão os testes são executados em paralelo usando threads
  cargo test -- --test-threads=1
    - executa de forma sequencial, usa apenas uma thread
  cargo test -- --show-output
    - força mostra a saida padrão quando passar no teste
  
  /// Running a Subset of Tests by Name
  
  cargo test one_hundred
    - executa só a função test que tiver esse nome

  /// Filtering to Run Multiple Tests
  
  cargo test add
    - executa os testes que tem add no nome
  cargo test soma
    - roda tudo que tem "soma" no nome
  cargo test matematica 
    - roda tudo no módulo matematica
  cargo test matematica::soma  
    - roda esse teste específico
  
  - se o nome de um modulo e nome de uma função em outro modulo for igual vai executar ambos


  /// onde os testes devem ficar?
    - **Unitário** → mesmo arquivo, dentro de `mod tests`
    - **Integração** → pasta `tests/` na raiz do projeto
     A vantagem de ficar no mesmo arquivo é que o teste tem acesso a funções privadas, o que em Rust é importante por causa das regras de visibilidade.
*/

// src/lib.rs
pub fn add_two(a: usize) -> usize {
  a + 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_two_and_two() {
      let result = add_two(2);
      assert_eq!(result, 4);
  }

  #[test]
  fn add_three_and_two() {
      let result = add_two(3);
      assert_eq!(result, 5);
  }

  #[test]
  fn one_hundred() {
      let result = add_two(100);
      assert_eq!(result, 102);
  }
}


/*
  - Controlling How Tests Are Run 
    - Ignoring Some Tests Unless Specifically Requested

    - como executar apenas testes ignorados?
      cargo test -- --ignored
    - como executar testes nao ignorados e ignorados?
      cargo test -- --include-ignored
*/

// usar #[ignore]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}