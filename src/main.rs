fn main() {
    let input: u128 = parse_fib_input();
    let fib_number = fib(input);
    println!("F_{} = {}", input, fib_number);
}

fn parse_fib_input() -> u128 {
    println!("Which Fibonacci number do you want?");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line.trim().parse::<u128>().expect("Failed to parse number")
}

pub fn fib(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[test]
fn test_fib() {
    for &(x, y) in [(0, 1), (1, 1), (2, 2), (3, 3), (4, 5), (5, 8), (6, 13)].iter() {
        assert_eq!(fib(x), y);
    }
}
