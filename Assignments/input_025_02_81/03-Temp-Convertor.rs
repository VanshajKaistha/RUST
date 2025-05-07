use std::io;

fn main() 
{
    println!("Welcome to temperature convertor from F to C.\n");
    println!("Enter your temperature: ");
    let mut temp = String::new();
    io::stdin()
    .read_line(&mut temp).expect("Unable to read value");
    let fer:f32 = temp.trim().parse().expect("Unable to read fer value");
    let celcius: f32 = ((fer-32.0)*5.0)/9.0;
    println!("{}F is {} C",fer,celcius);
}
