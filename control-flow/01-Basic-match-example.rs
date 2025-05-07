#[allow(dead_code)]
fn main()
{
    let age = 18;
   //with range
   match age
    {
        0..=12=>println!("Kid"),
        13..=19=>println!("Teenager"),
        _=>print!("Adult"),
    };
    //with tuples
    let tup = (0,-1);
   match tup 
    {
        (0,0)=>println!("Kid"),
        (x,0)=>println!("at x-axis:{}",x),
        (0,y)=>println!("at y-axis:{}",y),
        _=>print!("Adult"),
    };
    //with enums
    enum Color
    {
        Red,
        Green,
        Blue,
    }
    let c = Color::Green;
   match c 
    {
        Color::Green=>println!("G"),
        Color::Red=>println!("R"),
        Color::Blue=>println!("B"),
    };

    //with Gaurds
    let num = 60;
    match num
    {
        x if x%2 == 0 => println!("Even"),
        _=>println!("Odd"),
    }
}
