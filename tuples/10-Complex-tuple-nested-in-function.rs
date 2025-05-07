fn analysis(input: &str)->((usize,usize),f64)
{
    let words:Vec<&str> = input.split_whitespace().collect();
    println!("{:?}",words);
    println!("{:?}",words.get(0));
    let tot_len:usize = words.iter().map(|w| w.len()).sum();
    let avg_len:f64 = tot_len as f64/ words.len() as f64;
    ((words.len(),tot_len),avg_len)
}

fn main()
{
    let sentence = "Rust is fast and safe";
    let ((word_c,char_c),avg_len) = analysis(sentence);
    println!("Word:{:?},Char:{:?},Word_len:{:.2}",word_c,char_c,avg_len);
}
