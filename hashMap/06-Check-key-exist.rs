use std::collections::HashMap;
fn main()
{
    let mut text = HashMap::new();
    text.insert("Rust","prog lang");
    if text.contains_key("Rust")
    {
        println!("Found in hash map.");
    }
    else
    {
        println!("Not found!");
    }
}
