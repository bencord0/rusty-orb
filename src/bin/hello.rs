use std::env;

fn main() {
    let to = env::var("PARAM_TO").unwrap_or("World!".to_string());
    println!("Hello {to}");
}
