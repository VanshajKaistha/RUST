// Taking array input in RUST
use std::io;
fn main()
{
    //Declare the size pof array
    println!("Enter the size of array:");
    let mut arr_size = String::new();
    io::stdin()
        .read_line(&mut arr_size)
        .expect("Read failed");

    let size: usize = match arr_size.trim().parse()
    {
        Ok(num)=> num,
            Err(_)=>
            {
                println!("Invalid size value");
                return;
            },
    };

    //dcalre a vector
    let mut arr:Vec<i32> = Vec::with_capacity(size);
    for mut i in 0..size
    {
        println!("Enter the {} element",i+1);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Read failed");
        match input.trim().parse::<i32>()
        {
            Ok(s)=> arr.push(s),
                Err(_)=> 
                {
                    println!("Invalid Input retry.");
                    i -=1; //Warning: value assigned to `i` is never read
                },
        }
    }
    println!("Array: {:?}",arr);
    let sum:i32 = arr.iter().sum();
    let avg:f64 = sum as f64 / size as f64; //using as to change the type of the variable
    println!("Sum: {}",sum);
    println!("Average: {}",avg);

    if let Some(max) = arr.iter().max()
    {
        println!("Max:{}",max);
    }
    if let Some(min) = arr.iter().min()
    {
        println!("Min:{}",min);
    }
}

