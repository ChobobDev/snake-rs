use cursive::views::{Dialog,TextView};

pub fn run() {
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );
    siv.run();
}
