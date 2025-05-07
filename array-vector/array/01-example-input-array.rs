// Taking array input in RUST
use std::io;
fn main()
{
//Declare the size pof array
    const SIZE:usize = 5;
    // array syntax is:
    //arr:[type;size] = [0;5]
    let mut arr:[i32;SIZE] = [0;SIZE];
    println!("Please enter {} integers.",SIZE);
    // use a loop to take input from the user as you do in c but first take the string and then convert them into int and use parse.
    
    for mut i in 0..SIZE
    {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Read failed");
        match input.trim().parse::<i32>()
        {
            Ok(s)=> arr[i] = s,
            Err(_)=> 
            {
                println!("Invalid Input retry.");
                i -=1; //Warning: value assigned to `i` is never read
            },
        }
    }
    println!("Array: {:?}",arr);
}

