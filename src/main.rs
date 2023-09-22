use std::collections::HashMap;

fn main() {
    let mut inventory = Inventory::new();

    inventory.add_product("Product1".to_string(), "Description1".to_string(), 10.0, 100);
    inventory.add_product("Product2".to_string(), "Description2".to_string(), 15.0, 50);

    inventory.sell_product("Product1", 20).unwrap_or_else(|err| {
        println!("Error: {}", err);
    });

    inventory.purchase_product("Product2", 30, 12.0);

    inventory.generate_inventory_report();
}

struct Product{
    name: String,
    description: String,
    price: f64,
    quantity:u32,
}

struct Sale{
    product:String,
    quantity:u32,
    sale_price:f64,
}

struct Purchase{
    product:String,
    quantity:u32,
    purchase_price:f64,
}

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory { products: HashMap::new()}
    }


fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32){
    let product = Product{ name:name.clone(), description, price, quantity };
    self.products.insert(name,product);
}

fn sell_product(&mut self, product_name: &str, quantity: u32) -> Result<(), String> {
    if let Some(product) = self.products.get_mut(product_name) {
        if product.quantity >= quantity {
            product.quantity -= quantity;
            Ok(())
        } else {
            Err("Not enough stock".to_string())
        }
    } else {
        Err("Product not found".to_string())
    }
}

fn purchase_product(&mut self, product_name: &str, quantity: u32, purchase_price: f64){
    if let Some(product) = self.products.get_mut(product_name){
        product.quantity += quantity;
        product.price = purchase_price; 
    }else{
        let product = Product{  name: product_name.to_string(), description: "".to_string(), price: purchase_price, quantity };
        self.products.insert(product_name.to_string(), product);
    }
}

fn generate_inventory_report(&self) {
    println!("Inventory Report:");
    println!("{:<10} {:<20} {:<10} {:<10}", "Name", "Description", "Price", "Quantity");
    for product in self.products.values() {
        println!("{:<10} {:<20} {:<10} {:<10}", product.name, product.description, product.price, product.quantity);
    }
}
}

