use std::io;
fn main()
{
    let vec:Vec<i32> = vec![1,2,3,4,5];
/*
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    let sum:i32  = a.trim().parse().expect("Input"); 
*/
    let sum:i32 = 0;
    let tot:i32 = vec.iter().map(|x|x+sum).collect();
    println!("{}",tot);
}
