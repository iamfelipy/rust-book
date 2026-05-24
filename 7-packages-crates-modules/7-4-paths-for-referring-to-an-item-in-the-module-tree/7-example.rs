/*
pergunta: 
se eu criar uma função dentro de `back_of_house`, ela consegue acessar `eat_at_restaurant` mesmo se não for `pub`?

resposta:
Sim, módulos filhos podem acessar itens do módulo pai, mesmo que não sejam `pub`.
*/

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}