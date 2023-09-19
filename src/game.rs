use crate::container::Container;

pub fn run() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    let container = Container::new();
    siv.add_layer(container);

    siv.run();
}
