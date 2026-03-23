### Mini Grep

#### objetivo:
- colocar em pratica o que foi aprendido no "the rust book"

#### descrição:
- Criação de uma versão simplificada da ferramenta grep (busca global de strings e impressão das linhas correspondentes).

#### funcionalidades:
- Buscar uma string dentro de um arquivo específico e imprimir as linhas que a contém.
- Realizar buscas que não diferenciam maiúsculas de minúsculas, controladas por uma variável de ambiente

#### fundamentos:    
- Utilização de recursos do terminal.
- Utilização de variáveis de ambiente para personalizar o comportamento da ferramenta.
- Impressão de mensagens de erro no stderr ao invés de stdout, permitindo redirecionar saídas de sucesso para arquivos sem perder os erros na tela.
- Aprender sobre estruturação de aplicações em Rust.
- Analise de argumentos de linha de comando.
- Conceitos aplicados ao longo do desenvolvimento:
  - Organização de código (Capítulo 7)
  - Utilização de vetores e strings (Capítulo 8)
  - Tratamento de erros (Capítulo 9)
  - Uso de traits e lifetimes quando apropriado (Capítulo 10)
  - Escrita de testes (Capítulo 11)
  - Introdução rápida a closures, iteradores e trait objects (abordados em mais detalhes nos Capítulos 13 e 18).
  
#### Principais comandos Rust úteis

> **Obs.:** Para saber como executar o projeto, leia também o README na raiz do repositório.

```sh
# Executar o projeto localmente
$ cargo run -- <termo_de_busca> <caminho_arquivo>

# Exemplo:
  $ cargo run -- query exemplo.txt
  # search case insensitive
  $ IGNORE_CASE=1 cargo run -- to poem.txt

# Rodar todos os testes do projeto
$ cargo test
```

