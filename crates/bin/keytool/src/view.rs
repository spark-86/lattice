use key::Key;

pub fn view(path: &String, show_secret: bool, show_rust: bool) {
    let key = Key::disk_get(path);
    println!("{}", key.pretty_format(show_secret));
    if show_rust {
        println!("Rust Array: {:?}", key.pk.unwrap());
    }
}
