use std::collections::HashMap;

fn main()
{
    let mut name = HashMap::new();
    name.insert("Vanshaj",26);
    name.insert("shaj",23);
    name.insert("Vanj",25);
    name.insert("Vans",65);

    match name.get("Vans")
    {
        Some(age)=> println!("Vans is {} old",age),
        None=> println!("Not found"),
    }
}
