use std::io;
use std::i128;

fn main() {
    let parsed_input:i32 = input("fib index:").parse().expect("not i32");
    dbg!(parsed_input);
    println!("awnser is: {}", fib(parsed_input));
}

fn fib(position: i32) -> i128 {
    let mut first = 1;
    let mut second = 1;
    let mut new = 0;
    for iter in 2..position {
        new = second + first;
        println!("iter:{iter}, fib: {new}");
        second = first;
        first = new;
    }
    new
}

fn input(question: &str) -> String{
    let mut typed_input = String::new();
    println!("{question}");
    io::stdin()
        .read_line(&mut typed_input)
        .expect("Failed to read input");

    typed_input.pop();
    typed_input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(100), 354224848179261915075);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(35), 9227465);
        assert_eq!(fib(155), 110560307156090817237632754212345);
        assert_eq!(fib(103), 1500520536206896083277);
    }
}
