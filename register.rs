 use dialoguer::Input::with_theme;

fn register_user() -> Result<String, String> {
    let username = with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a username: ")
        .interact_text()?;

    let password = with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a password: ")
        .with_password(true)  // Mask password input
        .interact_text()?;

    let confirm_password = with_theme(&ColorfulTheme::default())
        .with_prompt("Confirm password: ")
        .with_password(true)  // Mask password input
        .interact_text()?;

    // Validate password confirmation
    if password != confirm_password {
        return Err(String::from("Passwords do not match!"));
    }

    Ok((username, password))
}
fn hash_user_password()

fn store_registered_user_details()
