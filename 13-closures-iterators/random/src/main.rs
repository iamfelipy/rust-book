// # closure como variavel
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

// # diferentes formas de definir closures

//funcao
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
//diferentes formas de definir a closure
//closures
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
// corpo da closure tem apenas uma expressão
let add_one_v4 = |x|               x + 1  ;