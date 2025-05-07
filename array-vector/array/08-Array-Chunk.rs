fn main()
{
    let arr = [21,42,24,95,46];
    for chunk in arr.chunks(2)
    {
     println!("Array: {:?}",chunk);
    }
}
