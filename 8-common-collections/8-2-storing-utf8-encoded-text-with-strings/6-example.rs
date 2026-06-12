// Slicing Strings

- usar intervalo
  - retorna um &str
  - se o intervalo nao pegar a quantidade correta de bytes gera panic
  - Зд = 4 bytes
- let hello = "Здравствуйте";
- let s = &hello[0..4];

// Para ativar o backtrace ao rodar comandos cargo, use assim:
//
// $ RUST_BACKTRACE=1 cargo run
//
// Isso mostrará o rastreamento da pilha caso aconteça um panic.
//
// Você também pode usar com outros comandos cargo, como:
// $ RUST_BACKTRACE=1 cargo test