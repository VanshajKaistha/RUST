fn main()
{
    let mut arr = [21,42,13,24,95,46];
    arr.sort();
    println!("sorted Array: {:?}",arr);
    arr.sort_by(|a,b|b.cmp(a));
    println!("Desending order:{:?}",arr);
}
