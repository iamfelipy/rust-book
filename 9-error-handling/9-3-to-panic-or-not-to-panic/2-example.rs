/*
  ---- Creating Custom Types for Validation
*/

// exemplo: usuario adivinha um numero entre 1 e 100
// melhoria no codigo do capitulo 2

// sem tipo personalizado e validação não abstraida e encapsulada

// src/main.rs
loop {
  // --snip--

  let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
  };

  if guess < 1 || guess > 100 {
      println!("The secret number will be between 1 and 100.");
      continue;
  }

  match guess.cmp(&secret_number) {
      // --snip--
  }
}


// com tipo personalizado e modulo dedicado

// a usar Guess tira a quantidade de verificações em tempo de execução

//  src/guessing_game.rs
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// lib.rs
pub mod guessing_game;

// main.rs
use std::io;
use rand::Rng;
use std::cmp::Ordering;

use crate::guessing_game::Guess;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number!");

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        let guess: Guess = match guess_input.trim().parse::<i32>() {
            Ok(num) => match Guess::new(num) {
                guess => guess,
            },
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
