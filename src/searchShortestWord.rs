fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    let mut ans=10;
    let mut l = 10;
    let mut s="";
    for word in sentence.split_whitespace() {
        l = word.len();
        if ans > l {
            ans = l;
            s = word;
        }
    }
    println!("{s}")
}
