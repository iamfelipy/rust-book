use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //collect:  permitindo acesso indexado aos argumentos da linha de comando.
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Você consegue importar minigrep porque em projetos Rust, o próprio nome do pacote é usado como crate interno (a partir do nome no Cargo.toml). Assim, o código em src/lib.rs fica disponível via minigrep para o binário em src/main.rs
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}



