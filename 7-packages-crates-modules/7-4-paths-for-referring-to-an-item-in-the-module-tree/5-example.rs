/*
- Considere o código que modela a situação em que um chef corrige um pedido incorreto e o leva pessoalmente ao cliente.
- Acreditamos que o back_of_house módulo e a deliver_order função provavelmente manterão a mesma relação e serão movidos juntos caso decidamos reorganizar a árvore de módulos do crate. Portanto, usamo super para que tenhamos menos locais para atualizar o código no futuro, caso este código seja movido para um módulo diferente.
*/
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}