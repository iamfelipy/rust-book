// Strings Are Not So Simple

// Métodos úteis para trabalhar com String e &str

fn main() {
    let frase = String::from("Olá, mundo! Bem-vindo ao mundo do Rust.");

    // contains: verifica se uma substring está presente
    if frase.contains("Rust") {
        println!("A frase contém a palavra 'Rust'.");
    } else {
        println!("A frase NÃO contém a palavra 'Rust'.");
    }

    // replace: substitui uma substring por outra
    let nova_frase = frase.replace("mundo", "universo");
    println!("Original: {}", frase);
    println!("Modificada: {}", nova_frase);
}