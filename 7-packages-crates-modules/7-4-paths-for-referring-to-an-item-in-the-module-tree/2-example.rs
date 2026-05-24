// diferença entre crate:: e chamar o modulo no mesmo nivel

// cenario 1: mudando movendo front_of_house e eat_at_restaurant para customer_experience

    // O caminho absoluto começa em crate::
    // front_of_house não está mais na raiz
    // O relativo acompanha o “lugar atual”

    // src/lib.rs
    mod customer_experience {
        mod front_of_house {
            mod hosting {
                fn add_to_waitlist() {}
            }
        }
        //  o nome do módulo definido no mesmo nível da árvore de módulos que eat_at_restaurant
        pub fn eat_at_restaurant() {
            // ❌ absoluto QUEBRA
            // crate::front_of_house::hosting::add_to_waitlist();

            // ✅ relativo CONTINUA FUNCIONANDO
            front_of_house::hosting::add_to_waitlist();
        }
    }

// cenario 2: 
// movendo eat_at_restaurant para o modulo dining

    // src/lib.rs
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
    }

    mod dining {
        pub fn eat_at_restaurant() {
            // ✅ absoluto CONTINUA FUNCIONANDO
            crate::front_of_house::hosting::add_to_waitlist();

            // ❌ relativo QUEBRA
            // front_of_house::hosting::add_to_waitlist();
        }
    }

// hosting e add_to_waitlist são privados, não vai compilar