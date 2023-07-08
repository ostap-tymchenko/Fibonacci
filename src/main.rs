use std::i128;
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(Dialog::text("choose a mode:\n")
        .title("Fibinacci tools")
        .button("Calc by index", |siv| input(siv, Mode::CalcByIndex))
        .button("Calc by value", |siv| input(siv, Mode::CalcByValue))
        .button("Quit", |s| s.quit()));

    siv.run();
}

enum Mode {
    CalcByIndex,
    CalcByValue
}

fn input(siv: &mut Cursive, mode: Mode) {
    siv.pop_layer();
    let dialog = Dialog::around(EditView::new().with_name("input"))
        .title("Enter Text")
        .button("Submit", |s| {
            let input = s.call_on_name("input", |view: &mut EditView| view.get_content()).unwrap();
            dbg!(input);
            s.quit();
        });
    siv.add_layer(dialog);

    // if mode == Mode::CalcByIndex {
    //     fib_calc_by_index(input);
    // }
}

// fn input(siv: &mut Cursive, mode: Mode) {
//     siv.pop_layer();
//     let dialog = Dialog::around(EditView::new())
//         .title("Enter index or value")
//         .button("Submit", move |s| {
//             let input = s.call_on_name("input", |view: &mut EditView| view.get_content()).unwrap();
//             *input_text.borrow_mut() = input;
//             s.quit();
//         })
//     )
//
//     siv.add_layer(dialog);
//     if mode == Mode::CalcByIndex {
//         fib_calc_by_index(position)
//     }
// }

fn fib_calc_by_index(position: i32) -> i128 {
    match position {
        0 => 1,
        1 => 1,
        _ => {
            let mut prev_prev = 1;
            let mut prev = 1;
            let mut now = 0;
            for iter in 1..position {
                now = prev_prev + prev;
                prev_prev = prev;
                prev = now;
            }
            now
        }
    }
}

fn start_buffer(siv_window: &mut Cursive) {
    siv_window.add_layer(Dialog::text("loading"))
}

fn result(siv_window: &mut Cursive, awnser: i128) {
}
