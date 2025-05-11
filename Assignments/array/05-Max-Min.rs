fn main()
{
    let array:[i32;5] = [10,20,30,40,50];
    let max:i32 = *array.iter().max().unwrap();
    let min:i32 = *array.iter().min().unwrap();
    println!("max: {} & min: {}",max,min);
}
