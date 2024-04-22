fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 1 {
        arr[len / 2] as f64
    } else {
        let mid = len / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    }
}

fn main() {
    let arr = [1, 3, 6, 7, 9];
    
    println!("Median of arr: {}", find_median(&arr));
    
}
