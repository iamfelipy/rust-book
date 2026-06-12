quiz 1
  Qual é a diferença entre usar a + b e a.push_str(b) para concatenar duas strings?
  resposta: + consome a propriedade de a, enquanto push_str não.
  Contexto: “push_str” pega &mut self, enquanto + pega “self”, portanto + consome a propriedade e push_str não.

quiz 2
  Qual é o número máximo de vezes que uma alocação de memória dinâmica (heap) pode ocorrer neste programa? Escreva sua resposta em dígitos, por exemplo, 0 ou 1.

  ```rust
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = s1 + "-" + &s2 + "-" + &s3;
  ```

  resposta: 7
  Contexto: Uma alocação para cada chamada a String::from e uma alocação para cada vez que + é chamado.
  correção:
      - ele tenta reutilizar a mesma alocação
      - só aloca novamente no heap se a capacidade não for suficiente