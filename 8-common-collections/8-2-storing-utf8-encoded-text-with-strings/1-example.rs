// What Is a String?
fn main() {
    //  O 'static é apenas o tempo de vida mais longo possível de um &str.
    // String literal (&'static str)
    let literal: &'static str = "Este é um literal de string";
    
    // literal já é um &str. A atribuição só copia a referência, sem mover ownership.
    // Usar &literal e apenas literal são equivalentes nesse caso porque literal já é um &str. O operador & apenas referencia algo, mas se já é uma referência (&str), &literal vira &&str, que normalmente é ajustado automaticamente pelo compilador. Portanto, ambos resultam em um &str devido ao "deref coercion" do Rust.
    // String slice (&'static str)
    let s_slice_ref: &str = literal;

    // String (heap allocation)
    let s_string: String = String::from("Esta é uma String");

    // Exemplo de uso dos slices
    // greeting é um &str (string slice), não um String. Ele é um literal string imutável armazenado diretamente no binário, não uma string heap-allocada.
    let greeting = "你好，世界!"; 
    let slice: &str = &greeting[0..3]; // fatia dos primeiros 3 bytes (equivalente a um caractere chinês em UTF-8)
    
    println!("Literal de string: {}", literal);
    println!("Slice de string: {}", s_slice_ref);
    println!("String heap alocada: {}", s_string);
    println!("Fatia de string UTF-8 (primeiros três bytes): {}", slice);
}