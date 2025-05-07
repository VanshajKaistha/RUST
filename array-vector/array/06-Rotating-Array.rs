fn main()
{
    let mut arr = [1,2,3,4,5];
    arr.rotate_left(2); //this will rotate array to left by 2 steps.
    println!("Rotated Array: {:?}",arr);
    arr.rotate_right(3); //this will rotate array to left by 2 steps.
    println!("Rotated Array: {:?}",arr);
}
