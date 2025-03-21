use std::io;

fn main() {
    println!("Calculate the Nth fibonacci number. Enter a count: ");
    
    let mut input_count = String::new();
    io::stdin().read_line(&mut input_count).expect("Failed to read line");
    let input_count: u32 = input_count.trim().parse().expect("Please enter a number");

    println!("Fibonacci number {input_count} is: ");
    
    // Base case
    if input_count == 0 {
        println!("0");
        return;
    }

    let mut prev: u128 = 0;
    let mut curr: u128 = 1;
    
    for _ in 1..input_count {
        // Might overflow
        // Could look into .checked_add() but
        // let's keep it simple
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    println!("{curr}");
}
