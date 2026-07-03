Pergunta 1
Um programador Rust está projetando uma biblioteca para escrever interfaces de linha de comando. Como parte dessa biblioteca, ele está implementando uma função para analisar flags de linha de comando fornecidas por um usuário. Qual implementação seria mais apropriada para esse domínio?

```rust

fn parse_flag_v1(flag: &str) -> Result<String, String> {
  // a função strip_prefix em Rust serve exatamente para remover um prefixo específico de uma string.
  match flag.strip_prefix("--") {
    Some(no_dash) => Ok(no_dash.to_string()),
    None => Err(format!("Invalid flag {flag}"))
  }
}
fn parse_flag_v2(flag: &str) -> String {
  match flag.strip_prefix("--") {
    Some(no_dash) => no_dash.to_string(),
    None => panic!("Invalid flag {flag}")
  }
}
```

resposta:
parse_flag_v1

Contexto: Aqui, o programador provavelmente vai querer usar um erro recuperável (o Result). Se um usuário da CLI passar uma flag com formato incorreto, a biblioteca de CLI pode querer fornecer ajuda adicional, como exibir o conjunto possível de flags. Um panic forçaria a aplicação a mostrar apenas a mensagem de panic, e provavelmente seria uma experiência de usuário pior.