fn rotating_vector(vec:&mut Vec<i32>,k:usize)
{
    let len = vec.len();
    vec.rotate_left(k%len);
}

fn main()
{
    let mut vec = vec![1,2,3,4,5,6,7,8];
    rotating_vector(&mut vec,3);
    println!("Rotated vector: {:?}",vec);
}
