/*
- Test Organization
    - unit tests
      - Testing Private Functions
*/

/// Como discutimos em “Caminhos para Referir um Item na Árvore de Módulos”, itens em módulos filhos podem usar os itens em seus módulos ancestrais.
// internal_adder é privado

pub fn add_two(a: usize) -> usize {
  internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn internal() {
      let result = internal_adder(2, 2);
      assert_eq!(result, 4);
  }
}