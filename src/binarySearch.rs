fn main() {
    let a = [1,1,2,2, 2,3, 3, 4, 5];
    let target = 3;
    let mut low=0;
    let mut high = a.len()-1;
    let mut ans=a.len();
    while low<=high {
        let mut mid = low + (high-low)/2;
        if a[mid] >= target {
             ans = mid;
            high = mid-1;
        }else{
            low = mid+1;
        }
    }
    println!("{ans}");
 }
 
 