//quiz 1
pergunta: 
Qual é a ordem correta, onde "A contem B"?

resposta: 
package > crate > module
Contexto: Um pacote é a unidade organizacional de nível superior, contendo crates. Um crate contém módulos.

//quiz 2
pergunta: 
How many crates does this package contain? Write your answer 

// Estrutura de árvore dos arquivos que compõem o package "foobar":
//
// foobar
// ├── Cargo.toml
// ├── build.rs
// └── src
//     ├── main.rs
//     ├── util.rs
//     ├── lib.rs
//     └── bin
//         └── alt.rs
//
resposta: 3
contexto: main.rs gera um crate binário, lib.rs gera um crate de biblioteca, e bin/alt.rs gera um crate binário. O util.rs presumivelmente seria usado como um módulo dentro de um desses crates, mas não é um nome de arquivo especial reconhecido pelo Rust, portanto não seria seu próprio crate. build.rs é um script de build.