#[derive(Debug)]
struct Rectangle
{
    width:u32,
    height:u32,
}

impl Rectangle //definng a method
{
    //Within an impl block, the type Self is an alias for the type that the impl block is for. 
    //Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.
    fn area(&self)->u32
    {
        self.width*self.height
    }
}

fn main()
{
    let rect = Rectangle{width: 32, height: 50,};
    println!("Area of rectangle: {}",rect.area());
}
