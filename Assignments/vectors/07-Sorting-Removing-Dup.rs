use std::collections::HashSet;
fn uni_Element(vec:Vec<i32>)-> Vec<i32>
{
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}



fn main()
{
    let mut vec = vec![5,3,8,3,1,5,6,8];
    vec.sort();
    println!("{:?}",vec);
    let unique = uni_Element(vec);
    println!("{:?}",unique);
}
