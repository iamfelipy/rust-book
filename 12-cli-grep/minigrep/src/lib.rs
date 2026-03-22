// lib.rs
// Isso permite ter funções compartilhadas entre binário e biblioteca no mesmo projeto.
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub file_path: String,
}

impl Config {                                  // string literal
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
      
      if args.len() < 3 {
          return Err("not enough arguments");
      }

      let query = args[1].clone();
      let file_path = args[2].clone();

      Ok(Config { query, file_path })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // O operador ? retorna imediatamente do bloco atual caso o valor seja Err (ou None), propagando o erro para o chamador; caso contrário, desempacota o valor de sucesso.
  let contents = fs::read_to_string(config.file_path)?;

  for line in search(&config.query, &contents) {
    println!("{line}");
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}