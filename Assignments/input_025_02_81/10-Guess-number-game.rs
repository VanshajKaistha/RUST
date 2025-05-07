use rand::Rng; //need to create rand dependency 
use std::io;

fn main()
{
    println!("Guess a number:");
    let ran = rand::thread_rng().gen_range(1..=100);
    let mut chance = 0;
    loop
    {
        println!("Input a number:");
        let input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failure in reading line");

        let guess = match input.trim().parse()
        {
            Ok(s)=> s,
                _=> {
                    println!("Invalid input value");
                    return
                }
        };
        chance +=1;
        match guess.cmp(&ran)
        {
            std::cmp::Ordering::Less => println!("Too low! Try again."),
                std::cmp::Ordering::Greater => println!("Too high! Try again."),
                std::cmp::Ordering::Equal => {
                    println!("Congratulations! You guessed the number in {} attempts!", chance);
                    break;
                }
        }
    }
}
