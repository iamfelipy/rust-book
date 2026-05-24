mod front_of_house

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Sim, isso vai funcionar. O caminho absoluto crate::front_of_house::hosting::add_to_waitlist() está correto porque front_of_house é um módulo do crate e add_to_waitlist está acessível via pub use no submódulo hosting.

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // Não vai funcionar. Como front_of_house está declarado como mod dentro de lib.rs, mas não está em escopo direto nesta função (nem foi feito use do módulo inteiro, apenas do hosting via pub use), o caminho relativo correto seria só hosting::add_to_waitlist().

    //
    // Peça um café da manhã no verão com torrada de centeio.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Mudamos de ideia sobre qual pão gostaríamos.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // A linha a seguir não irá compilar se a descomentarmos; não temos
    // permissão para ver ou modificar a fruta sazonal que acompanha a refeição.
    // meal.seasonal_fruit = String::from("blueberries");

    //

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}