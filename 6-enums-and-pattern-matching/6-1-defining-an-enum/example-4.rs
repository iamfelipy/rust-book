//v1
enum Option<T> {
    None,
    Some(T),
}

let some_number = Option::Some(5);
let some_char = Option::Some('e');

//v2
// precisa tipar aqui
let absent_number: Option<i32> = Option::None;

//v3
// Geralmente, isso ajuda a capturar um dos problemas mais comuns com null: assumir que algo não é null quando na verdade é.
let x: i8 = 5;
let y: Option<i8> = Option::Some(5);

// Isso não compila: não é possível somar i8 com Option<i8>
// let sum = x + y;

// Para somar, precisamos primeiro extrair o valor de Option<i8> usando match ou if let, por exemplo:

let sum = match y {
    Option::Some(val) => x + val,
    Option::None => x, // ou algum outro comportamento apropriado
};

println!("Sum: {}", sum);