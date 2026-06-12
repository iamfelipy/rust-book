
// pergunta 1

//// Reverses the elements of a vector in-place
fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        std::mem::swap(&mut v[i], &mut v[n - i - 1]);
    }
}

// If you tried to compile this program, which of the following best describes the compiler error you would get?

resposta:
cannot borrow v as mutable twice for v[i] and v[n - i - 1]

por que: o compilador não considera o valor específico dos índices usados para acessar um array, então &mut v[i] e &mut v[n - i - 1] são assumidos como possivelmente referindo-se ao mesmo elemento. Portanto, obtemos um erro em que v não pode ser emprestado mutavelmente duas vezes.


// pergunta 2

// Normally if you try to compile this function, the compiler returns the following error:
mensagem:
// Erro[E0499]: não é possível emprestar *v como mutável mais de uma vez ao mesmo tempo
error[E0499]: cannot borrow `*v` as mutable more than once at a time
   --> test.rs:5:40
    |
5   |         std::mem::swap(&mut v[i], &mut v[n - i - 1]);
    |                                        ^second mutable borrow occurs here
    |                             ^ first mutable borrow occurs here
    |         |
    |         first borrow later used by call

pergunta:
- (1) passaria pelo compilador e (2) possivelmente causaria comportamento indefinido se executado?
- Marque cada programa que satisfaça ambos os critérios OU marque "Nenhum destes programas" se nenhum for satisfatório.

resposta: 
None of these programs
porque: 
Esta função não pode causar uma violação de segurança de memória porque i != n - i - 1 para todo i, então as duas referências mutáveis sempre se referem a elementos diferentes.

// pergunta 3

- Das seguintes correções (destacadas em amarelo), qual delas melhor satisfaz estes três critérios:
- A função corrigida passa no compilador Rust,
- A função corrigida preserva a intenção do código original e
- A função corrigida não introduz ineficiências desnecessárias.

resposta:
fn reverse_unsafe(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
/*yellow -*/ let p1 = &mut v[i] as *mut String;
/*yellow -*/ let p2 = &mut v[n - i - 1] as *mut String;
/*yellow -*/ unsafe { std::ptr::swap_nonoverlapping(p1, p2, 1); }
    }
}
/*
Contexto: Em uma situação em que o borrow checker rejeita uma operação que na verdade é segura e não há solução alternativa,
então código unsafe às vezes é aceitável se for crítico evitar alocações.
Neste caso específico, você deveria realmente usar Vec::swap, que é implementado internamente com código unsafe amplamente testado semelhante ao código acima.
Mas, em geral, se a biblioteca padrão não suportar seu caso de uso, então unsafe pode ser aceitável se usado corretamente.
*/

// Solução utilizando Vec::swap (recomendada)
fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        v.swap(i, n - i - 1);
    }
}

// Exemplo de uso de Vec::swap
fn main() {
    let mut v = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
    ];
    reverse(&mut v);
    println!("{:?}", v); // Output: ["d", "c", "b", "a"]
}