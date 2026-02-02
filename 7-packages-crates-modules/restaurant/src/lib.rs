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

    // Relative path
    front_of_house::hosting::add_to_waitlist();

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