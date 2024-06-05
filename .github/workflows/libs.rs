
#[derive(Debug)]
struct Product {
    id: i32,
    name: String,
    description: String,
    price: f32,
    quantity: i32,
}

pub struct ProductStore {
    products: HashMap<u32, Product>, 
}

impl ProductStore {
    pub fn new() -> Self {
        ProductStore {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), String> {
        if self.products.contains_key(&product.id) {
            return Err(format!("Product with ID {} already exists", product.id));

        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn edit_product(&mut self, product_id: u32, new_name: Option<String>, new_price: Option<f32>, new_description: Option<String>) -> Result<(), String> {
        let mut product = self.products.get_mut(&product_id).ok_or_else(|| format!("Product with ID {} not found", product_id))?;

        if let Some(name) = new_name { 
            product.name = name;
        }

        if let Some(price) = new_price {
            product.price = price;
        }

        if let Some(description) = new_description {
            product.description = description;
        }

        Ok(())
    }

    pub fn delete_product(&mut self, product_id: u32) -> Result<(), String> {
        self.products.remove(&product_id).ok_or_else(|| format!("Product with ID {} not found", product_id))?;
        Ok(())
    }
}
