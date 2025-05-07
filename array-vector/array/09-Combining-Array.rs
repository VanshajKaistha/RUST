fn main()
{
    let arr1 = [1,2,3];
    let arr2 = [4,5,6];
    let combo: Vec<i32> = arr1.iter().chain(arr2.iter()).cloned().collect();
    println!("Combined arr: {:?}",combo);
}
