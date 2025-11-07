fn main() {
    
    // SECTION IF
    // if em rust, bloco de codigo, arms(é bloco de codigo associado ao if, match)

    // example 1
    // if em rust, bloco de codigo, arms(bloco de codigo associado ao if, match)
    // project branchs
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("conditions was false");
    }

    //exanpe 2
    // a condição do if precisa ser um bool
    // como em ruby e javascript, rust não converte automaticamente outros tpos para booleanos
    // gera erro
    // error[E0308]: mismatched types
    // expected `bool`, found integer
    let number2 = 3;
    if number2 {
        println!("number was three");
    }

    //example 3
    // multiplas condições com else if
    // multi else if fica dificil de ler, match pode ajudar a resolver isso
    let number3 = 6;

    if number3 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number3 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number3 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 , or 2");
    }

    // example 4
    // if em uma declaração let
    // os dois braços(arms) devem retornar o mesmo tipo
    let conditions = true;
    let number4 = if conditions { 5 } else { 6 };

    println!("the value of number is: ${number4}");

    
    // SECTION LOOP

    // example 1
    // loop
    // use "break" para sair
    // ctrl + c no terminal para parar
    // use "continue" para ir proxima iteração

    loop {
        println!("again");
    }

    //example 2
    // atribuindo valores a variaveis com loop
    // o valor após break vira o valor resultante do loop.

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // para atrbiuir no final esse ; tem que existir
            break counter * 2;
        }
    };

    // example 3
    // loops aninhados e labels loop
    // consigo dizer a quem o break pertence
    // os labels, rotulos sao usado para referenciar o loop no break e continue, util em loops aninhados
    'outer: loop {
        loop {
            break 'outer;
        }
    }

    
    // SECTION WHILE loops condicionais

    // example 1
    // while - loop condicional
    let mut number5 = 5;
    while number5 != 0 {
        println!("{number5}!");
        number5 -= 1;
    }
    println!("LIFTOFF!!!");

    
    // SECTION for
    
    // example 1
    // iterando coleções de forma insegura com while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }
    // examplo 2
    // iterando de coleções de forma segura com for
    // recomendado
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{element}");
    }

    // exemplo 3
    // countdown com for + range + rev
    // Iteradores são objetos que permitem percorrer uma sequência de valores um por um
    // (1..4) não é uma tupla, é um Range (std::ops::Range), que é um iterador de números (1, 2, 3).
    // .rev() inverte a ordem do iterator (ficando 3, 2, 1).
    // Dentro de (1..4) só tem o iterator de números nesse intervalo (não inclui o 4).
    // O tipo de dado de (1..4) é Range<i32>.
    // Sim, .rev() é um método disponível para iteradores, incluindo Range.
    // Outros métodos comuns de Range (via Iterator) incluem:
    // .next(): pega o próximo valor.
    // .collect(): transforma em coleção (ex: Vec).
    // .map(): aplica função a cada elemento.
    // .filter(): filtra elementos.
    // .sum(): soma todos elementos.
    // Todos são herdados do trait Iterator, que Range implementa.
    // Além de Range, outros iteradores comuns em Rust incluem:
    // Iteradores de array/slice (.iter(), .iter_mut())
    // Iteradores de Vec, HashMap, HashSet
    // Iteradores criados com .map(), .filter(), .enumerate(), .zip(), .chain()
    // std::iter::repeat, std::iter::once, std::iter::empty
    // Todos esses implementam o trait Iterator.
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");


}