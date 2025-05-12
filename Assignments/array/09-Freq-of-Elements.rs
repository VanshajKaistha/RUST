fn main() {
    let arr = [1, 2, 2, 3, 3, 3];
//    arr.sort();

    let mut i = 0;
    while i < arr.len() {
        let mut count = 1;
        while i + 1 < arr.len() && arr[i] == arr[i + 1] {
            count += 1;
            i += 1;
        }
        println!("{} appears {} times", arr[i], count);
        i += 1;
    }
}

