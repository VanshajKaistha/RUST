fn main()
{
    let arr = [1,2,3,4,5];
    let sq: Vec<i32> = arr.iter().map(|&x|x*x).collect();
    println!("Square array: {:?}",sq);
}
