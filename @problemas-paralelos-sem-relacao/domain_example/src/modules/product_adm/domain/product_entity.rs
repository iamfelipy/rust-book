#[derive(Debug)]
pub struct Product {
  id: String,
  name: String,
  description: String,
  purchase_price: f64,
  stock: u32,
  // coloquei com tipo primitivo para não criar acoplamento com lib externa
  // representa o timestamp
  created_at: i64,
  updated_at: i64,
}

impl Product {
  pub fn id(&self) -> &str {
    &self.id
  }
  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }
  pub fn name(&self) -> &str {
    &self.name
  }
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
  pub fn description(&self) -> &str {
    &self.description
  }
  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }
  pub fn purchase_price(&self) -> f64 {
    self.purchase_price
  }
  pub fn set_purchase_price(&mut self, purchase_price: f64) {
    self.purchase_price = purchase_price;
  }
  pub fn stock(&self) -> u32 {
    self.stock
  }
  pub fn set_stock(&mut self, stock: u32) {
    self.stock = stock;
  }
  pub fn created_at(&self) -> i64 {
    self.created_at
  }
  pub fn set_created_at(&mut self, created_at: i64) {
    self.created_at = created_at;
  }
  pub fn updated_at(&self) -> i64 {
    self.updated_at
  }
  pub fn set_updated_at(&mut self, updated_at: i64) {
    self.updated_at = updated_at;
  }
}

impl Product {
  pub fn new(
    id: String,
    name: String,
    description: String,
    purchase_price: f64,
    stock: u32,
    created_at: i64,
    updated_at: i64,
  ) -> Self {
    Self {
      id,
      name,
      description,
      purchase_price,
      stock,
      created_at,
      updated_at,
    }
  }
}
