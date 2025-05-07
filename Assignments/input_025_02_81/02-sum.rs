use std::io;
fn main()
{
    let mut a = String::new();
    println!("Enter 1st value: ");
    io::stdin()
    .read_line(&mut a).expect("Failed to take input");
    let val1:u32 = a.trim().parse().expect("Value not convereted");
    println!("{}",val1);
    println!("Enter 2nd value: ");
    let mut b = String::new();
    io::stdin()
    .read_line(&mut b).expect("Failed to take input");
    let val2:u32 = b.trim().parse().expect("Value not convereted");
    println!("{}",val2);
    println!("Sum of both values: {}",val1+val2);
}

