mod modules;

use crate::modules::product_adm::usecase::create_product_usecase::CreateProductUsecase;
use crate::modules::product_adm::repository::product_repository::ProductRepository;

fn main() {
  let product_repository = ProductRepository{};
  let create_product_usecase = CreateProductUsecase::new(product_repository);
  create_product_usecase.execute(
    "1".to_string(),
    "Produto 1".to_string(),
    "Descrição do Produto 1".to_string(),
    100.0,
    10,
    1679700000,
    1679703600,
  );
}