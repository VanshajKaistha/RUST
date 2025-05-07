fn info(person: (&str,i32))
{
    println!("Name:{},Age:{}",person.0,person.1);
}

fn main()
{
    let person = ("Vanshaj",26);
    info(person);
}
