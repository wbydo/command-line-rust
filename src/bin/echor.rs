use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Hibiki Sakuraoka <wbydo@users.noreply.github.com>")
        .about("Rust echo")
        .get_matches();
    println!("{:?}", std::env::args());
}
