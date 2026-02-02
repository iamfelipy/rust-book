pub struct ProductRepository;

impl ProductRepository {
    pub fn create(
        &self,
        id: &str,
        name: &str,
        description: &str,
        purchase_price: f64,
        stock: u32,
        created_at: i64,
        updated_at: i64,
    ) {
      println!("product created in repository");
    }
}