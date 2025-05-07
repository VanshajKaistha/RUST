fn calculate(data: &[i32])->(i32,i32,f64)
{
    let sum:i32 = data.iter().sum();
    let mini:i32 = *data.iter().min().unwrap();
    let avrg:f64 = sum as f64/data.len() as f64;
    (sum,mini,avrg)
}

fn main()
{
    let num = [10,20,30,40,50,60];
    let (total,min,avg) = calculate(&num);
    println!("SUM:{},MIN:{},AVG:{:.2}",total,min,avg);
}
