//use std::collections;
enum Oprtion
{
    Add(i32,i32),
    Sub(i32,i32),
    Mul(i32,i32),
}

fn execute(op:Oprtion)->i32
{
    match op
    {
        Oprtion::Add(a,b)=>a+b,
        Oprtion::Sub(a,b)=>a-b,
        Oprtion::Mul(a,b)=>a*b,
    }
}

fn main()
{
    let operations = [Oprtion::Add(5,6),Oprtion::Sub(5,4),Oprtion::Mul(66,7)];
    for op in operations
    {
        println!("Result:{}",execute(op));//clone doesn't work here as clone is not part of that enum.
        //println!("Result:{}",execute(op.clone()));//clone doesn't work here as clone is not part of that enum.
        //you cannot pass &op.clone() as well
        //borrowing doesn't worked here.
    }
}
