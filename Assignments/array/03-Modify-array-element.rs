fn main()
{
    let mut arr:[i32;5] = [10,20,30,40,50];
    for a in arr
    {
        println!("{}",a);
    }
    arr[2] = 100;
    println!("{:?}",arr);
}
