use std::io;
fn main()
{
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    let age  = a.trim().parse().expect("Input"); 
   // let age: u32 = a.trim().parse().expect("Input"); 
    println!("Age Entered: {}",age);

    match age
    {
        // this is wrong use range.
        0..=19 => println!("teenager"),
        20..=33 => println!("young"),
        34..=50 => println!("Middle aged"),
        51..=60 => println!("Mature"),
        61..= 70 => println!("Retied"),
        71..=100 => println!("Old"),
        _ => println!("Unknown age"),
    }

}
