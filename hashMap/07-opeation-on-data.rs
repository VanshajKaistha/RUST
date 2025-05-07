use std::collections::HashMap;
fn main()
{
    let mut fruit = HashMap::new();
    fruit.insert("Apples",1);
    fruit.insert("Grapes",2);
    fruit.insert("Pineapple",3);
    println!("{:?}",fruit);
    fruit.remove("Apples");//removing element
    for (key,val) in &fruit
    {
        println!("{}:{}",key,val);//output varies in terms     of sequence
    }
    fruit.entry("Grapes").or_insert(10);//using default value

    println!("{:?}",fruit);
}
