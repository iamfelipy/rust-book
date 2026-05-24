//glob
use std::collections::*; // importa todos os itens públicos de collections

fn main() {
    // agora posso usar HashMap e VecDeque sem importação individual
    let mut map = HashMap::new();
    map.insert("a", 1);

    let mut queue = VecDeque::new();
    queue.push_back(2);

    println!("{:?} {:?}", map, queue);
}

// Cuidado: usar glob pode dificultar manter o controle do que está em escopo e tornar o código mais suscetível a conflitos de nomes.