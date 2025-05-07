fn main()
{
    let vec = vec![10,20,30];
    
    for val in &vec
    {
        println!("{}",val);
    }
//iter
    vec.iter().for_each(|val| println!("by iter:{}",val));
//method

    let square:Vec<i32> = vec.iter().map(|x|x*x).collect();

    println!("method:{:?}",square);
}

