use std::io;

fn main() {
    println!("Enter sentence:");
    loop{
        let mut snt = String::new();
        io::stdin().read_line(&mut snt).expect("Failed to read line");
        let snt = snt.trim().to_string();
        let length = calculate_length(&snt);
        println!("The length of sentence you enter is: {}", length);
    }
}

fn calculate_length(s: &String) -> usize{
    s.len()
}
