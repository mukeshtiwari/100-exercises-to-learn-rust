// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.

pub struct Order {
  product_name : String, 
  quantity : u32, 
  unit_price : u32
}


impl Order {
  pub fn new(product_name : String, quantity : u32, unit_price : u32) -> Order {
    validate_product_name(&product_name);
    valid_quantity(&quantity);
    valid_unit_price(&unit_price);
    Order { 
      product_name, 
      quantity, 
      unit_price }
  }
  pub fn product_name(&self) -> &String{
    &self.product_name
  }

  pub fn quantity(&self) -> &u32 {
    &self.quantity
  }

  pub fn unit_price(&self) -> &u32 {
    &self.unit_price
  }

  pub fn total(&self) -> u32 {
    self.quantity * self.unit_price
  }

  pub fn set_product_name(&mut self, name : String) {
    validate_product_name(&name);
    self.product_name = name 
  }

  pub fn set_quantity(&mut self, quant : u32) {
    valid_quantity(&quant);
    self.quantity = quant
  }

  pub fn set_unit_price(&mut self, uprice : u32) {
    valid_unit_price(&uprice);
    self.unit_price = uprice
  }

}

//   The product name can't be empty and it can't be longer than 300 bytes.

pub fn validate_product_name(product_name : &String) {
  if product_name.is_empty() {
    panic!("Title cannot be empty")
  }
  if product_name.len() > 300 {
    panic!("Title cannot be longer than 300 bytes");
  }
}

//   The quantity must be strictly greater than zero.

pub fn valid_quantity(quantity : &u32) {
  if *quantity == 0 {
    panic!("quantity should be greater than 0")
  }
}

//   The unit price is in cents and must be strictly greater than zero.

pub fn valid_unit_price(uprice : &u32){
  if *uprice == 0 {
    panic!("unit price should be greater than 0")
  }
}
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.



//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
