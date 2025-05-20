fn main()
{
    let mut vec = vec![];
    println!("Empty vector {:?}",vec);
    println!("Pushing vector:");
    vec.push(5);
    vec.push(10);
    vec.push(15);
    vec.push(20);
    println!("{:?}",vec);
    vec.pop();
    println!("Poping vector:");
    println!("{:?}",vec);
}
