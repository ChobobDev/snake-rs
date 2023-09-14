use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, SelectView};
use cursive::Cursive;

pub fn run() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    let screen = SelectView::<String>::new()
        .with_name("screen")
        .fixed_size((100, 50));

    siv.add_layer(Dialog::around(LinearLayout::horizontal().child(screen)).title("Snake-rs"));

    siv.run();
}
