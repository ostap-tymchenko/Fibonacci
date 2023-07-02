use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;

pub fn ui () {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::text("choose a mode:\n")
        .title("Fibinacci tools")
        .button("Find by index", |s| s.quit())
        .button("Find by value", |s| s.quit())
        .button("Quit", |s| s.quit()));

    siv.run();
}
