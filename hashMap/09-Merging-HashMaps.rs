use std::collections::HashMap;
fn main()
{
    let mut fruit = HashMap::new();
    fruit.insert("Apples",1);
    fruit.insert("Grapes",2);
    fruit.insert("Pineapple",3);

    let mut fruit_new = HashMap::new();

    fruit_new.insert("Mango",4);
    fruit_new.insert("Pine",5);
    for (key,val) in fruit_new
    {
        fruit.entry(key).or_insert(val);
    }
    println!("{:?}",fruit);
}
