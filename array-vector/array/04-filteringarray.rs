fn main()
{
    let arr = [1,2,3,4,5,6];
    //filter
    let evens:Vec<i32> = arr.iter().filter(|&x|x%2 == 0).cloned().collect();
    println!("Original Array: {:?}",arr);
    println!("Filtered Array: {:?}",evens);
}
