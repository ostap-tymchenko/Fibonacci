use std::io;
mod ui;

#[allow(dead_code, unused_variables, unused_mut)]
fn main() {
    let mode: Mode;
    let mode_input = input("find fibinachi number by index or, index of fib number inputed? (1/2) :");

    if mode_input == "1" {
        mode = Mode::ByIndex;
    } else if mode_input == "2" {
        mode = Mode::ByFib;
    }


    let parsed_input:i32 = input("fib index (i64 max):").parse().expect("not i32");
    dbg!(parsed_input);
    println!("the {parsed_input} fibinachi number is: {}", fib(parsed_input));
}

enum Mode {
    ByIndex,
    ByFib
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


