use std::collections::HashMap;
fn frequency(vec:Vec<i32>)->HashMap<i32,i32>
{
    let mut map = HashMap::new();
    for num in vec
    {
        *map.entry(num).or_insert(0)+=1;   
    }
    map
}

fn main()
{
    let vec = vec![1,2,3,4,5,1,2,3,4,5];
    let freq = frequency(vec);
    println!("{:?}",freq);
}
