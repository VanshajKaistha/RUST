/*
 * this is a simple function example
fn main()
{
    let w = 30;
    let h = 40;

    println!("Area of rectangle {}",area(w,h));
}

fn area(w: u32, h:u32)->u32
{
    w*h
}
*/
/*
 * this is Using tuples
fn main()
{
    let rect = (30,40);

    println!("Area of rectangle {}",area(rect));
}

fn area(d: (u32,u32))->u32
{
    d.0*d.1
}
*/

/*
 * this is Using struct */
struct Rectangle
{
    width:u32,
    height:u32,
}

fn main()
{
    let rect = Rectangle
    {
        width:30,
        height:50,
    };
    println!("Area = {}",area(&rect));
}
fn area(area: &Rectangle)->u32 //this is borrowing method we did not move the ownership.
{
    area.width*area.height
}
