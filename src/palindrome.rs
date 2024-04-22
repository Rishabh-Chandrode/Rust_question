fn main() {
    let mut input = String::new();
    println!("Enter a string: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut chars: Vec<char> = input.trim().chars().collect();

    if is_palindrome(&chars) {
        println!("{} is a palindrome", input);
    } else {
        println!("{} is not a palindrome", input);
    }
}

fn is_palindrome(input: &Vec<char>) -> bool {

    let mut i = 0;
    let mut l = input.len()-1;

    while i < l {

        if input[i] != input[l] { return false; }

        i +=1;
        l-=1;
    }

    true
}