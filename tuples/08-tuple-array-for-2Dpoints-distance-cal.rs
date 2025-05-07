//distance is a function where we have two points p1 and p2.
//p1 and p2 are tuples.
//func returns f64.
fn distance(p1:(f64,f64),p2:(f64,f64))->f64
{
    let dx =p2.0 -p1.0;
    let dy =p2.1 -p1.1;
    (dx*dx+dy*dy).sqrt()
}
fn main()
{
    let point = [(0.0,0.0),(3.0,4.0),(6.0,8.0)];
    println!("len:{}",point.len());
    for i in 0..point.len()-1
    {
        println!("Here");
        let dis = distance(point[i],point[i+1]);
        println!("Distance b/w: {:?} & {:?} is {:.2}",point[i],point[i+1],dis);

    }
}
