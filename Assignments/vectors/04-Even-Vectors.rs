fn main()
{
    let vec = vec![1,2,3,4,5,6,7,8];
    let even: Vec<i32> = vec.into_iter().filter(|x|x%2 == 0).collect();
    println!("{:?}",even);
}
