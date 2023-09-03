use cursive::views::{Dialog,TextView};

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );
    siv.run();
}
