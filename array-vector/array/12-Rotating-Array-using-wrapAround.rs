//This program rotates an array to the right or left with wrap-around logic.
//It handles both positive and negative rotations.

fn rotate_array<T: Clone>(arr:&[T],k:i32)->Vec<T>
{
    let len = arr.len();
    if len == 0
    {
        return vec![];
    }
    //Normalise rotation to ensure it is in array bounds
    let k = ((k%len as i32)+ len as i32)%len as i32;
    let k = k as usize;
    //Perform rotation
    let mut rotated = Vec::with_capacity(len);
    rotated.extend_from_slice(&arr[len-k..]);//Add the last k elemnt
    rotated.extend_from_slice(&arr[..len-k]);//add the first len-k element
    rotated
}

fn main()
{
    let arr = [1,2,3,4,10];
    let k = 2; //normal condition

    let rotate = rotate_array(&arr,k);
    println!("Rotated array right by {}: {:?}",k,rotate);
    
    let rotate_left = rotate_array(&arr,-2);
    println!("Rotated array left by 2: {:?}",rotate_left);
}
