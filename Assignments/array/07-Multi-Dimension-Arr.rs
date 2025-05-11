fn main()
{
    let matrix:[[i32;3];2] = [[1,2,3],[4,5,6]];
    println!("matrix:{:?}",matrix);
    let sum:i32 = matrix.iter().flat_map(|row| row.iter()).sum();
    println!("{}",sum);
}
