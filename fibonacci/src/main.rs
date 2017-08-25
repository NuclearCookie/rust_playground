use std::io;

fn main() {
    println!("Fibonacci number: ");
    let mut result = String::new();
    let nth_number: i32;
    loop {
        io::stdin().read_line(&mut result).expect(
            "Failed to read line",
        );
        let potential_nth_number = result.trim().parse();
        if potential_nth_number.is_err() {
            println!("Please input a number");
        } else {
            nth_number = potential_nth_number.unwrap();
            break;
        }
    }

    println!("Fibonacci @{}: {}", nth_number, fibonacci(nth_number));
}

fn fibonacci(nth: i32) -> i32 {
    let mut result = 1;
    let mut prev_result = 0;
    for _ in 1..nth {
        let old = result;
        result += prev_result;
        prev_result = old;
    }
    result
}
