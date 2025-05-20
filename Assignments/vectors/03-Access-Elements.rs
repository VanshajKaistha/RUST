fn main()
{
    let vec = vec![1,2,3,4,5];
    println!("Accessing vector's 1st and 3rd element: {} & {}.",vec[0],vec[2]);
    match vec.get(1)
    {
        Some(val)=>println!("Accessing vector's 2nd element: {}.",val),
        None=>println!("Out of bound"),
    }
}
