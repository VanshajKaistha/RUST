fn main()
{
    for i in 0..=5
    {
        println!("Number{:}",i);
    }
    println!("*********Example with Collections***********");
    let fruit = ["Apple","Banana", "Orange","Grape"];
    for fruit in fruit.iter()
    {
        println!("Fruit:{}",fruit);
    }
    println!("*********Iterating with Index***********");
    let fruit = ["Apple","Banana", "Orange","Grape"];
    for (index,fruit) in fruit.iter().enumerate()
    {
        println!("Index:{} Fruit:{}",index,fruit);
    }
}
