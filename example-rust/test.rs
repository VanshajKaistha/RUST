use std::io;
fn main()
{
    let mut mystr = String::new();
    io::stdin()
    .read_line(&mut mystr)
    .expect("Error fail to read line");
    let score:i32 = (mystr as i32)
    .trim()
    .parse().expect("reason");
}
