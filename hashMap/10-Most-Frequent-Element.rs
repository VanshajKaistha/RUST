use std::collections::HashMap;

fn main()
{
    let num:Vec<i32> = vec![1,2,2,3,3,3,4,4,4,4];
    let mut count_map = HashMap::new();
    for numb in num
    {
        let count = count_map.entry(numb).or_insert(0);
        *count += 1;
    }

    let most_freq = count_map.iter().max_by_key(|&(_,count)|count).unwrap();
    println!("Most frequent num: {}({}times)",most_freq.0,most_freq.1);
}
