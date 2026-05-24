Imagine a Rust package (crate) called `foobar` with the directory structure:

foobar
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── engine.rs
    └── engine/
        └── analysis.rs

File contents:

// engine/analysis.rs
pub fn run() {}

// engine.rs
mod analysis;
pub use analysis::*;

// lib.rs
pub mod engine;

Digamos que outro desenvolvedor Rust está usando o crate da biblioteca foobar em um pacote separado e quer chamar a função run. 

Qual caminho ele escreveria?
A resposta correta é:
foobar::engine::run

Contexto: A árvore de módulos gerada por esta estrutura de diretórios é a seguinte:

foobar
└── engine
└── run
Portanto, o caminho para run é foobar::engine::run.
