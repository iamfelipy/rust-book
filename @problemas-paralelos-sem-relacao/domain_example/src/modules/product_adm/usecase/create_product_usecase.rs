use super::repository::product_repository::ProductRepository;
use super::domain::product_entity::Product;

pub struct CreateProductUsecase {
    product_repository: ProductRepository,
}

impl CreateProductUsecase {
    pub fn new(product_repository: ProductRepository) -> Self {
        Self { product_repository }
    }

    pub fn execute(
        &self,
        id: String,
        name: String,
        description: String,
        purchase_price: f64,
        stock: u32,
        created_at: i64,
        updated_at: i64,
    ) {
        let product = Product::new(
            id,
            name,
            description,
            purchase_price,
            stock,
            created_at,
            updated_at,
        );
        println!("entity product domain created");
        self.product_repository.create(
            product.id(),
            product.name(),
            product.description(),
            product.purchase_price(),
            product.stock(),
            product.created_at(),
            product.updated_at(),
        );

        println!("usecase called");
    }
}


// snake_case: para nomes de variáveis, funções, campos de struct, nome de arquivos.
// CamelCase: para nomes de structs, enums, traits e tipos.
// SCREAMING_SNAKE_CASE: para constantes e estáticos.