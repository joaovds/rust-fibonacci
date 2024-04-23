use std::io;

fn main() {
    let mut nth = String::new();

    println!("Please, input a number:");
    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line!");

    let nth: u64 = nth.trim().parse().expect("Invalid value!");

    println!("Fibonacci result: {}", fibonacci(nth))
}

fn fibonacci(nth: u64) -> u64 {
    if nth <= 1 {
        return nth;
    }

    let mut penult_result = 0;
    let mut last_result = 1;
    let mut result = 0;

    for _ in 2..=nth {
        result = penult_result + last_result;
        penult_result = last_result;
        last_result = result;
    }

    result
}
