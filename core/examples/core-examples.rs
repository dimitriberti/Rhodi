use rhodi_core::{TracedDocument};

fn main() {
    // let mut manuscript = String::new();

    // manuscript..push_str("# My First Manuscript\n");

    print!("\nExample of the core library\n");
    let manuscript = TracedDocument::new("My First Manuscript", "This is the content of my first manuscript   using the core library.");
    println!("{:#?}", manuscript);
}