use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter 5 number to do average:");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read");
    let nums:Vec<f32> = input.trim()
    .split_whitespace()
    .map(|s|s.parse().expect("Failed to parse"))
    .collect();
    let avg:f32 = nums.iter().sum::<f32>()/5.0;
    println!("{}",avg);
} 

