// Output screen sizes in WIDTHxHEIGHT format

extern crate xcb;
use std::process;

fn main() {
    let connection =
        match xcb::Connection::connect(None) {
            Ok((conn, _)) => conn,
            Err(_) => {
                println!("Could not connect to X server");
                process::exit(1);
            },
        };

    let setup = connection.get_setup();

    let screen_iterator = setup.roots();

    for screen in screen_iterator {
        let width = screen.width_in_pixels();
        let height = screen.height_in_pixels();
        println!("{}x{}", width, height);
    }
}
