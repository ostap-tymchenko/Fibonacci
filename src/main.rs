use std::io;
fn main() {
    let parsed_input:i32 = input("fib index (i64 max):").parse().expect("not i32");
    dbg!(parsed_input);
    println!("awnser is: {}", fib(parsed_input));
}

fn fib(position: i32) -> i64 {
    let mut second = 1;
    let mut first = 1;
    let mut new = 0;
    for iter in 0..position {
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
