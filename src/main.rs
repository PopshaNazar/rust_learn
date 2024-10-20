fn main() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
