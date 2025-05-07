use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read");
    let num:i32 = match input.trim().parse()
    {
        Ok(s) => s,
        _ =>{println!("Invalid input"); return},
    };
    if num%2 == 0
    {
        println!("Even");
    }
    else
    {
        println!("Odd");
    }
    
    println!("Result: {}",num);
    
}

