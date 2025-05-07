use std::collections::HashSet;
fn main()
{
    let arr = [1,2,3,4,4,4,5,5,6,5];
    let unq:Vec<i32> = arr.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
    //hash set removes dups automatically but the order of the printed i not maintained to print
    //ordered array use <BTreeSet<_>>
    println!("Unique: {:?}",unq);
}
