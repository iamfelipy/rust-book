// ----   Propagating Errors

// ---- introduction

/*
    exemplo: read_username_from_file
    sem usar atalho para propagação de erro
*/

use std::fs::File;
use std::io::{ self, Read };
                                                 // struct Error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut username = String::new();

    // read_to_string vem da trait Read
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        // ultima expressão da função, não preciso de return explicito
        Err(e) => Err(e),
    }
}

// --- A Shortcut for Propagating Errors: The ? Operator
// reduzindo o código anterior usando o operador ?

/*
    exemplo: Uma função que retorna erros para o código que a chamou usando o ? operador
*/

// src/main.ts
use std::fs::File;
use std::io::{ self, Read };

fn read_username_from_file() -> Result<String, io::Error> {
    // O ? chama From::from automaticamente
    // io::Error implementa from
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/*
    EXAMPLE FROM CUSTOMERROR
    condição para o ? funcionar
    o segundo generic do result precisa ter a trait From para cada 
*/

// esta no preludio, exemplo da declaração
// From é um trait genérico:
trait From<T> {
    fn from(value: T) -> Self;
}

use std::io;

#[derive(Debug)]
enum OurError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Json(serde_json::Error),
}

impl From<std::io::Error> for OurError {
    fn from(e: std::io::Error) -> OurError {
        OurError::Io(e)
    }
}

impl From<std::num::ParseIntError> for OurError {
    fn from(e: std::num::ParseIntError) -> OurError {
        OurError::Parse(e)
    }
}

impl From<serde_json::Error> for OurError {
    fn from(e: serde_json::Error) -> OurError {
        OurError::Json(e)
    }
}

use std::fs::File;
use std::io::{ self, Read };

fn read_username_from_file() -> Result<String, OurError> {

		// let n: i32 = texto.parse()?;        // ParseIntError
		// let f = File::open("a.txt")?;       // io::Error
		// let v = serde_json::from_str(s)?;   // serde_json::Error

    // o que acontece por baixo dos panos ao usar ?
    //Err(e)  →  Err(OurError::from(e))
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


/*
    exemplo:  Encadeamento de chamadas de método após o ? operador

    tornando mais curto
*/

use std::fs::File;
use std::io::{ self, Read };

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    // read_to_string retorna um Result<usize, io::Error>, onde usize é o número de bytes lidos.
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}


/*
    exemplo:  tornando ainda mais curto
*/

use std::fs;
use std::io;
// Ler um arquivo para uma string é uma operação bastante comum, então a biblioteca padrão fornece a fs::read_to_string função conveniente que abre o arquivo, cria uma nova string String, lê o conteúdo do arquivo, coloca o conteúdo nessa string String e a retorna. Claro, usar fs::read_to_string não nos dá a oportunidade de explicar todo o tratamento de erros, então fizemos isso da maneira mais longa primeiro.
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}




