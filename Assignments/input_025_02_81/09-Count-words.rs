use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a sentence:");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read");
    let count  = input.trim().split_whitespace().count(); //white space handles the space and allows us to use the count method to count strings
    println!("Count: {}",count);
    
}

