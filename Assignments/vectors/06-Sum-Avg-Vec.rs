fn main()
{
    let vector = vec![10,20,30,40,50];
    let sum:i32= vector.iter().sum();
    let average: f32 = sum as f32 / vector.len() as f32;
    println!("Sum of Elelments:{}",sum);
    
    println!("Average:{}",average);
}
