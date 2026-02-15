use crate::cli::keys::generate_key;

pub fn run(name: Option<String>, show: bool) -> crate::error::Result<()> {
    let name = name.unwrap_or_else(|| "default".to_string());

    generate_key(&name, show)?;

    if !show {
        println!("Key '{}' created successfully.", name);
        println!(
            "Run 'rhodi keygen --name {} --show' to see the public key.",
            name
        );
    }

    Ok(())
}
