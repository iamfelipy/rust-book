// let e variáveis mutaveis
// sombras(shadowing) de variaveis
// match e o tipo Result
// Métodos e funções associativas (String::new, rand::thread_rng)
// uso de crates externas com o Cargo
// loop e controle de fluxo (break, continue)

// traz a biblioteca "io" de entrada e saida para o escopo
// "std" é uma biblioteca padrão
use std::io;
// lib para gerar numeros aleatorios
// Rng é um trait, permiti usar gen_range
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    // usado para repetir instrução
    loop {
	    println!("Guess the number!");
    
	    // thread_rng() criar um gerador aleatório local à thread
	    let secret_number = rand::thread_rng().gen_range(1..=100);
	    
	    println!("The secret number is: {secret_number}");
			// macro !
	    println!("Please input your guess.");
			// new é uma função associativa
			// mut para variavel mutavel
	    let mut guess = String::new();
	    io::stdin()
	        // passando uma referência mutavel, read_line pode mudar o valor
	        .read_line(&mut guess)
	        // read_line retorna um Result, pode ser Ok() ou Err()
	        // o expect é enviado para o Err, se der erro
	        .expect("Failed to read line");
	    
	    // preciso converter guess string para funcionar com cmp
	    // trim: remove espaço e quebra de linha \n
	    // parse: converte string em numero
	    // expect(): trata o erro caso a conversão falhe
	    // u32: inteiro sem sinal de 32 bits
	    // shadowing(sombrar): reutilizar variavel e mudar o tipo
	    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
	    // match com o Result retornado por parse()
	    let guess: u32 = match guess.trim().parse() {
		    Ok(num) => num,
		    Err(_) => continue
	    };
	    
	    // Os {} são placeholders usados para inserir valores na string.
	    // let x = 5;
	    // let y = 10;
	    // println!("x = {x} and y + 2 = {}", y + 2);
	    println!("You guessed: {guess}");
	    
	    // match, tipo o switch
	    // guess.cmp retorna um Ordering
	    // Ordering é um enum com 3 variantes
	    // =>, isso separa o padrão(esquerda) da ação(à direita)
	    // Ordering::Less, padrão de correspondencia
	    match guess.cmp(&secret_number) {
		    Ordering::Less => println!("Too small!"),
	      Ordering::Greater => println!("Too big!"),
	      Ordering::Equal => {
		      println!("You win!");
		      break;
	      },
	    }
    }
}