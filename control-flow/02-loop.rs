fn main()
{
    let mut count = 0;
    loop
    {
        count +=1;
        if count == 5
        {
            break;
        }
        println!("{}",count);
    }
    println!("Broke the loop");

}
