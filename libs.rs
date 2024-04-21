
#[derive(Debug)] // Makes it easy to print for Debugging
struct Product {
    id:i32,
    name: String,
    description: String,
    price: f32,
    quantity:i32,
}

fn add_product(new_product:Product) ->Result <()> {
    
    println!("Adding product{:?}" new_product);
    Ok(())
}