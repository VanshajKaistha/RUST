use std::io;
fn main()
{
    //loop
   // {

        let mut a = String::new();
        io::stdin()
            .read_line(&mut a)
            .expect("Failed to read line");
        println!("value of a: {}",a.trim());

        let mut b = String::new();
        io::stdin()
            .read_line(&mut b)
            .expect("Failed to read line");
        println!("value of b: {}",b.trim());

        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read line");
        println!("value of opt: {}",opt.trim());

    //}
}
