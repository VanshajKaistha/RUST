use std::collections::HashSet;
fn uniq_elmnt(vec:Vec<i32>)->Vec<i32>
{
    let set:HashSet<_> = vec.into_iter().collect();//need to understand hash 
    let unique: Vec<i32> = set.into_iter().collect();
    return unique;
}

fn main()
{
    let vec = vec![12,12,11,21,44,21];
    let uniq = uniq_elmnt(vec);
    println!("{:?}",uniq);
}
