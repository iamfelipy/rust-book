/*
exemplo: lib.rs que fornece funcionalidades de um restaurante

- No ramo da restauração, algumas áreas de um restaurante são chamadas de " frente da casa" e outras de "bastidores" .
- A frente da casa é onde os clientes ficam; isso engloba o local onde os recepcionistas acomodam os clientes, os garçons anotam os pedidos e recebem os pagamentos, e os bartenders preparam as bebidas.
- Os bastidores são onde os chefs e cozinheiros trabalham na cozinha, os lavadores de pratos limpam e os gerentes realizam o trabalho administrativo.
*/

// criar a library restaurant
// cargo new restaurant --lib


// filename: src/lib.rs
//  Um front_of_house módulo que contém outros módulos que, por sua vez, contêm funções.
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}