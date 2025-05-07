#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle
{
    width:u32,
    height:u32,
}

impl Rectangle //definng a method
{
    //we can use a field within a method of the same name for any purpose.
    fn width(&self)->bool
    {
        self.width>0
    }
}

fn main()
{
    let rect = Rectangle{width: 32, height: 50,};
    if rect.width()
    {
        println!("value of width {}",rect.width);
    }
}
