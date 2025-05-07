// Taking array input in RUST
use std::io;
fn main()
{
//Declare the size pof array
    const SIZE:usize = 5;
    // array syntax is:
    //arr:[type;size] = [0;5]
    let mut arr:[i32;SIZE] = [0;SIZE];
    // use a loop to take input from the user as you do in c but first take the string and then convert them into int and use parse.
    
    for i in 0..SIZE
    {
        println!("Enter the {} element",i+1);
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Read failed");

        arr[i] = input.trim().parse().expect("Invalid input");
    }
    println!("Array: {:?}",arr);
}

