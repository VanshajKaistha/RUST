fn main()
{
    let arr1:[i32;5] = [1,2,3,4,5];
    let arr2:[i32;5] = [4,2,1,5,3];

    if arr1.len() != arr2.len()
    {
        println!("not a rotation of array");
    }
    else
    {
        let slice1 = &arr1[0..3];// todo
        println!("Slice value: {:?}", slice1);

        //sliding window method where a window of certain size is made which then check if the
        //value is present in the other object which will determine rotation of the arr1.
        let doubled = arr1.iter().chain(arr1.iter());
        let doubled_vec: Vec<_> = doubled.cloned().collect();

        if doubled_vec.windows(arr2.len()).any(|window| window == arr2)
        {
                println!("Yes(it's a rotation)");
        }
        else
        {
                println!("NO!! rotation");
        }
    }

}
