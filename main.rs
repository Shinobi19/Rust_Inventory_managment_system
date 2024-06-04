use std::iter::Product;

use dialoguer::{theme::ColorfulTheme, Select}; // Or another text UI library
use dialoguer::{Input::theme::ColorfulTheme}; // For Input
use rand::Rng; 
use src::libs;

fn main() {

    let name: String = Input::with_theme(&ColorfulTheme::default());
            .with_prompt("Enter the name of the product");
            .interact_text()
            .unwrap();

    let description: String = Input::with_theme(&ColorfulTheme::default());
            .with_prompt("Enter the description of the product");
            .interact_text()
            .unwrap();    
    
    let quantity: i32 = loop {
         match Input::<i32>::with_theme(&ColorfulTheme::default());
            .with_prompt("Enter the quantitiy");
            .interact_text()
        Ok(quantity) => break quantity,
        Err(err) => print!("Invalid quantity format, Please enter a whole number"),
    } 

    
    let price: f32 = loop {
          match Input::<f32>::with_theme(&ColorfulTheme::default());
          .with_prompt("Enter the price");
          .interact_text()
        Ok(price) => break price,
        Err(err) => print!("Invalid price format, Please enter a number"),
    } 

    let new_product = Product{
        id: rand::thread_rng().gen_rng(1...1000),
        name,
    }
    // Basic security: Replace this with a proper login system later
    if !prompt_for_login() { 
        println!("Invalid credentials!");
        return; 
    }

    loop { // Main program loop
        let options = &["Manage Inventory", "Record Sales", "Record Purchases", "Generate Reports", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action:")
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => inventory_menu(),  // define this later
            1 => record_sale(),     // define this later
            // ... (Add cases for other options) 
            _ => break,             // Exit
        }
    }
}

// Placeholder for a login function
fn prompt_for_login() -> bool { 
    // ...  temporary login code
    true // Temporary - will need proper authentication
}
