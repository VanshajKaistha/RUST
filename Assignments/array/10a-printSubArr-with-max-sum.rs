fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    let mut start = 0;
    let mut end = 0;
    let mut temp_start = 0;

    for i in 1..arr.len() {
        if current_sum + arr[i] < arr[i] {
            current_sum = arr[i];
            temp_start = i;
        } else {
            current_sum += arr[i];
        }

        if current_sum > max_sum {
            max_sum = current_sum;
            start = temp_start;
            end = i;
        }
    }

    let subarray = &arr[start..=end];
    println!("Maximum Subarray Sum: {}", max_sum);
    println!("Subarray: {:?}", subarray);
}
