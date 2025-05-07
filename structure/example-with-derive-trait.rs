#[derive(Debug)] //derive trait
struct Rectangle
{
    width:u32,
    height:u32,
}
#[derive(Debug)] //derive trait
#[allow(dead_code)]
struct Square
{
    side:u32,
}
fn main()
{
    let rect = Rectangle
    {
        width:30,
        height:50,
    };
    let sq = Square
    {
        side:30,
    };
    println!("debug(non pretty): {rect:?}"); //non pretty format
    println!("debug(pretty): {rect:#?}");//pretty format
    println!("debug(pretty): {sq:#?}");//pretty format
    println!("Area = {}",area(&rect));
}
fn area(area: &Rectangle)->u32 //this is borrowing method we did not move the ownership.
{
    area.width*area.height
}
