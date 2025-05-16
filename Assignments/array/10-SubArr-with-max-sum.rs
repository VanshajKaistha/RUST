//This code only prints the highest sum of a sub array not the sub-array.
//the complexicity is to much so i have skipped it for now to add the code of printing sub-array.

fn max_sub_array(arr:&[i32])-> i32
{
    if arr.is_empty()
    {
        return 0;
    }

    let mut max_curr = arr[0];
    let mut max_final = arr[0];

    for &n in &arr[1..]
    {
        max_curr = n.max(max_curr + n);
        max_final = max_final.max(max_curr);
    }
    max_final
}



fn main()
{
    let arr:[i32;9] = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_sub_array(&arr);
    println!("MAX SUM OF SUB-ARRAY: {}",max_sum);
}
