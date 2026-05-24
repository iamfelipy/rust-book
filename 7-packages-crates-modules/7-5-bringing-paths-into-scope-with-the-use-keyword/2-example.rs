// problema: não compila
// "use" só é valido no escopo daquele modulo, submodulos criam um novo escopo
// - nesse exemplo o hosting foi acessado em um escopo diferente do use
// Nome do arquivo: src/lib.rs
    mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
    }

    use crate::front_of_house::hosting;

    mod customer {
      pub fn eat_at_restaurant() {
          hosting::add_to_waitlist();
      }
    }

// solução 1: super
    // Nome do arquivo: src/lib.rs
    mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
    }

    use crate::front_of_house::hosting;

    mod customer {
      pub fn eat_at_restaurant() {
          super::hosting::add_to_waitlist();
      }
    }

// solução 2: mover use
// Nome do arquivo: src/lib.rs
    mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
    }


    mod customer {
      use crate::front_of_house::hosting;
      pub fn eat_at_restaurant() {
          hosting::add_to_waitlist();
      }
    }