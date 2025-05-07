use std::io;
fn main()
{
    let mut input = String::new();
    println!("Enter 2 number and operator of the operation you want to perform:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let inputs:Vec<&str> = input.trim()
        .split_whitespace()
        .collect();

    let a:i32 = inputs[0].parse().expect("Ivalid parse");
    let b:i32 = inputs[1].parse().expect("Ivalid parse");
    let oprt = inputs[2];

    let result = match oprt
    {
        "+" => a+b,
        "-" => a-b,
        "*" => a*b,
        "/" => a/b,
        _=> {
            println!("Invalid operator");
            return // as println! returns i32, it was incompatible with the match type that is causing error
            },
    };
    println!("Result: {}",result);
} 
