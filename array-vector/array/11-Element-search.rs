fn main()
{
    let arr = [10,3,44,30,11];
    let element = 3;
    match arr.iter().position(|&x|x == element)
    {
        Some(index)=>println!("Element:{},Index:{}",element,index),
        None=>println!("Target not found!"),
    }
}
