fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let original = "abcdefgh";
    let reversed = reverse_string(original);
    println!("Original string: {}", original);
    println!("Reversed string: {}", reversed);
}
