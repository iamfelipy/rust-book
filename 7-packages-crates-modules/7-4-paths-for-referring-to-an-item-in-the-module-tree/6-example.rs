/*
- No Exemplo 7-9, definimos uma struct pública back_of_house::Breakfast com um campo público toast, mas um campo privado seasonal_fruit. Isso modela o caso em um restaurante onde o cliente pode escolher o tipo de pão que acompanha a refeição, mas o chef decide qual fruta acompanha a refeição com base no que está na temporada e em estoque. As frutas disponíveis mudam rapidamente, então os clientes não podem escolher a fruta nem sequer ver qual fruta receberão.
- Além disso, observe que, por conta de back_of_house::Breakfast ter um campo privado, a struct precisa fornecer uma função associada pública que construa uma instância de Breakfast (aqui a chamamos de summer). Se Breakfast não tivesse tal função, não poderíamos criar uma instância de Breakfast em eat_at_restaurant porque não conseguiríamos definir o valor do campo privado seasonal_fruit em eat_at_restaurant.
*/

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
}

pub fn eat_at_restaurant() {
  // Peça um café da manhã no verão com torrada de centeio.
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Mudamos de ideia sobre qual pão gostaríamos.
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
  // A linha a seguir não irá compilar se a descomentarmos; não temos
  // permissão para ver ou modificar a fruta sazonal que acompanha a refeição.
  // meal.seasonal_fruit = String::from("blueberries");
}