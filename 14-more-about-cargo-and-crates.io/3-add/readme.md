criar workspace raiz /add com cargo.toml [workspace]

criar crate de biblioteca
cargo new add_one --lib

criar crate binario
cargo new adder
  isso adiciona como membro no cargo.toml do workspace

adicionar add_one como dependencia de adder em adder/cargo.toml
[dependencies]
add_one = { path = "../add_one" }

Execute o comando `cargo build` no diretório raiz `add`!

Execute o crate binário a partir do diretório add.
cargo run -p adder

executar no diretório add de nível superior. Executar cargo test em um workspace estruturado como este rodará os testes de todos os crates no workspace:
cargo test
cargo test -p add_one