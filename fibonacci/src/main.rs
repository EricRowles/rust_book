use std::io;

fn main() {
    println!("####################");
    println!("Fibonacci calculator");
    println!("####################");
    print_menu();
}

fn print_menu() {
    println!("The Fibonacci sequence is a sequence in which each number is the sum of the two preceding ones");
    println!("The standard sequence is 1, 1, 2, 3, 5, ...");
    println!("You will be asked for the first two numbers in the sequence and the number in the sequence you want returned.");

    let mut prev = 1;
    let mut curr = 1;
    println!(
        "Enter the first number in the sequence (default {}): ",
        prev
    );
    prev = get_number(prev);
    println!(
        "Enter the second number in the sequence (default {}): ",
        curr
    );
    curr = get_number(curr);
    println!("Starting numbers are {prev} and {curr}");
    println!("Enter which number you want to calculate (default 3): ");
    let n = get_number(3);
    if n == 0 {
        println!("There's no index 0 in the Fibonacci sequence...");
        return;
    }
    let fib = get_fibonacci(&mut prev, &mut curr, n);
    println!("Fibonacci number at index {n} is {fib}")
}

fn get_number(default: u64) -> u64 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    if number.trim() == "" {
        return default;
    }

    match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(
                "{} is not a number. Using default ({default})",
                number.trim()
            );
            default
        }
    }
}

fn get_fibonacci(prev: &mut u64, curr: &mut u64, n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return *prev;
    } else if n == 2 {
        return *curr;
    };

    for _ in 2..n {
        (*prev, *curr) = (*curr, *prev + *curr);
        // println!("{}", *curr);
    }
    *curr
}
