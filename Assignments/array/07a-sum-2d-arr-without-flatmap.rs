fn main()
{
    let matrix:[[i32;3];2] = [[1,2,3],[4,5,6]];
    println!("matrix:{:?}",matrix);
    let mut sum = 0;
    for row in matrix.iter()
    {
        for val in row.iter()
        {
            sum += val;
        }
    }
    println!("{}",sum);
}
