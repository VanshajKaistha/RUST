fn main()
{
    let studs = vec![("Alice",20),("Vanshaj",26),("Rajat",26)];
    for stud in studs
    {
        println!("Name:{}, Age:{}",stud.0,stud.1); //stud.0 means name, sttud.1 means age
    }
}
/*
 * Name:Alice, Age:20
Name:Vanshaj, Age:26
Name:Rajat, Age:26*/
