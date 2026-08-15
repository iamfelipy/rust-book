O texto discute a função find_nth em Rust, destacando problemas ao tentar ordenar um slice imutável (erro de compilação ao chamar sort em &elems), e como corrigir isso criando um Vec<&T> (vetor de referências) para ordenar sem copiar dados. Também explica quando usar Vec<&T> versus Vec, dependendo se o tipo implementa Copy, visando preservar eficiência e intenção de função somente-leitura.

Question 1
// Adicionar nth em um find remete à intenção de obter o enésimo elemento encontrado, ou seja, o elemento na posição n após aplicar algum critério de pesquisa ou ordenação; por exemplo, nth(2) retorna o terceiro elemento (índice começa em 0).
// Significa "n-ésimo", ou seja, qualquer posição "n" em uma sequência (por exemplo, terceiro, décimo, centésimo, etc).

fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}

// ao usar & em um array (&v), ele automaticamente vira um slice (&[T]).
// Um slice em Rust é uma referência a uma sequência contínua de elementos em uma coleção (como um vetor ou array), permitindo acesso a parte dos elementos sem copiar ou possuir os dados.
// Para criar um slice de uma posição até outra: let slice = &array[inicio..fim];
// Um slice ([T]) não é uma struct, mas um tipo primitivo fat pointer; o método sort(&mut self) é definido via trait (SliceExt) e recebe &mut self, mas slices não armazenam dados como structs tradicionais.

// If you tried to compile this program, which of the following best describes the compiler error you would get?

// answer: cannot borrow elems as mutable for sort
// context: O método slice::sort espera uma referência mutável para um slice, mas recebeu uma referência imutável.

-----------------------------
Question 2
/// Returns the n-th largest element in a slice
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}
// Normally if you try to compile this function, the compiler returns the following error:
--> test.rs:3:5
  |
3 |     elems.sort();
  |     ^^^^^^^^^^^^ `elems` is a `&` reference, so the data it refers to can not be borrowed as mutable
// Suponha que o compilador NÃO rejeitou esta função. Quais (se houver) dos seguintes programas (1) seriam aceitos pelo compilador e (2) possivelmente causariam comportamento indefinido se executados? Marque cada programa que satisfaça ambos os critérios, OU marque "Nenhum desses programas" se nenhum os satisfizer.

// answered: None of these programs
// context: Este programa é tecnicamente seguro em termos de memória porque slice::sort só pode mover elementos, não liberá‑los. Por exemplo, &v[0] é garantido apontar para algum número após chamar find_nth, mesmo que não seja o número original. 
// Note que find_nth(&v, 10) não causa comportamento indefinido porque Rust realiza verificações em acessos a arrays, então avaliar &v[10] irá provocar um panic.

-----------------------------
Question 3
/// Returns the n-th largest element in a slice
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}
Das correções a seguir (realçadas em amarelo), qual correção atende melhor a estes três critérios:

A função corrigida compila com o compilador Rust,
A função corrigida preserva a intenção do código original, e
A função corrigida não introduz ineficiências desnecessárias

resposta:
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    // iter() cria um iterador sobre referências dos elementos do slice (&T), e 
    // collect() transforma esse iterador em um Vec<&T>. 
    // Isso resulta em um vetor de referências para os elementos originais, sem copiá-los.  
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    return t.clone();
}

Contexto: Uma função como find_nth é claramente destinada a ser uma função somente de leitura, ou seja, extrair alguma propriedade da sequência de entrada. Qualquer solução que mutile ou descarte a entrada, portanto, não preserva a intenção original da função, mesmo que seja mais eficiente do que criar um vetor auxiliar.

Criar um Vec<&T> é preferível a criar um Vec<T>, pois se T for grande, então elems.to_vec() poderia ser custoso. No entanto, se soubéssemos que T: Copy, então to_vec seria preferível para reduzir o número de desreferências de ponteiro dentro de elems.sort().

Use Vec<&T> para tipos grandes, pois evita cópia custosa.
Use Vec<T> (com Copy) para tipos pequenos, pois é mais rápido acessar valores diretamente, sem desreferenciação.
Tipos Copy são simples, como números, bool, char e tuplas desses. Tipos com ponteiros não são Copy.