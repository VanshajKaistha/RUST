use std::io;
fn main()
{
    let mut number = String::new();
    println!("Enter Your Number:");
    io::stdin()
        .read_line(&mut number)
        .expect("Error: Not Able To Read");
    let score: i32 = number
        .trim()
        .parse()
        .expect("Please Enter The Valid Number");
    let grade  = 
    if score>=90
    {
        "A"
    }
    else if score>=80
    {
        "B"
    }
    else if score>=70
    {
        "C"
    }
    else if score>=60
    {
        "D"
    }
    else
    {
        "F"
    };
    println!("Your Grade Is {}",grade);
}

