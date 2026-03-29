// lib.rs
// Isso permite ter funções compartilhadas entre binário e biblioteca no mesmo projeto.
use std::error::Error;
use std::{env, fs};

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(
      mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
      // o primeiro valor no retorno de env::args é o nome do programa.
      args.next();

      let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
      };

      let file_path = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a file path"),
      };

      let ignore_case = env::var("IGNORE_CASE").is_ok();

      Ok(Config {
          query,
          file_path,
          ignore_case,
      })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // O operador ? retorna imediatamente do bloco atual caso o valor seja Err (ou None), propagando o erro para o chamador; caso contrário, desempacota o valor de sucesso.
  let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
      //O método contains(&query) aceita &str, então &query (que é &String) é automaticamente convertido para &str por coerção de referência.
      if line.to_lowercase().contains(&query) {
          results.push(line);
      }
  }

  results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
      // AAA (Arrange-Act-Assert) resumo rápido:
      // - Arrange: configurar os dados, variáveis ou contexto inicial do teste.
      // - Act: executar a funcionalidade a ser testada.
      // - Assert: verificar se o resultado é o esperado.
      let query = "rUsT";
      let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

      assert_eq!(
          vec!["Rust:", "Trust me."],
          search_case_insensitive(query, contents)
      );
  }
}