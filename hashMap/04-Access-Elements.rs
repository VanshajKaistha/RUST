use std::collections::HashMap;

fn main()
{
    let mut fruits = HashMap::new();
    fruits.insert("Apple",5);
    fruits.insert("Cake",1);
    if let Some(&count) = fruits.get("Aple")
    {
        println!("Apple count:{}",count);
    }
    else
    {
        println!("Not found");
    }
}
