fn return_tup()->(&'static str,i32,f64)
{
    ("Vanshaj",26,5.5)
}

fn main()
{
    let person = return_tup();
    println!("Name:{},age:{},height:{}",person.0,person.1,person.2);
}
