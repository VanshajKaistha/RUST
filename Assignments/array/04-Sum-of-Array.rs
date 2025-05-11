fn main()
{
    let arr:[i32;5] = [10,20,30,40,50];
    println!("{:?}",arr);
    let sum:i32 = arr.iter().sum();
    println!("{}",sum);
}
