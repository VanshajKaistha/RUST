// this method works for integers only
use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter 3 number:");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read");
    let nums:Vec<i32> = input.trim()
    .split_whitespace()
    .map(|s|s.parse().expect("Failed to parse"))
    .collect();
    if let Some(&largest) = nums.iter().max() //iter uses the method max for finding the largest element in the given nums
    {
        println!("Largest:{}",largest);
    }
    else
    {
        println!("Largest num not found");
    }
}

