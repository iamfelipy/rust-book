//v1
// não compila
fn get_or_default(arg: &Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    /*
    A função Option::unwrap espera self, ou seja, espera a posse de arg. No entanto, arg é uma referência imutável para uma option, então não pode fornecer a posse da option. Portanto, o compilador reclama que não podemos mover para fora de arg via unwrap.
    */    
    let s = arg.unwrap();
    s.clone()
}

//v2 - cenarios em que se o codigo fosse executado geraria comportamento indefinido
fn get_or_default(arg: &Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}

let opt = Some(String::from("Rust"));
get_or_default(&opt);
println!("{:?}", opt);

let opt = Some(String::from("Rust"));
let s = get_or_default(&opt);
println!("{}", s);

let opt = Some(String::from("Rust"));
get_or_default(&opt);

//v3 solução - idiomatica
// A função corrigida passa no compilador Rust,
// A função corrigida preserva a intenção do código original
// A função corrigida não introduz ineficiências desnecessárias.
// match combina is_none e unwrap, e já passa a referencia
fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone(),
    }
}