extern crate xcb;
use std::process;

fn main() {
    let conn: xcb::Connection =
        match xcb::Connection::connect(None) {
            Ok((conn, _)) => conn,
            Err(_) => {
                println!("Could not connect to X server");
                process::exit(1);
            },
        };

    let setup: xcb::Setup = conn.get_setup();
    let mut iter: xcb::ScreenIterator = setup.roots();
    let screen: xcb::Screen = match iter.next() {
        Some(scr) => scr,
        None => { println!("No screen found"); process::exit(1); },
    };

    println!("Information of screen {}", screen.root());
    println!("  Width..........: {}", screen.width_in_pixels());
    println!("  Height.........: {}", screen.height_in_pixels());
    println!("  White pixels...: {}", screen.white_pixel());
    println!("  Black pixels...: {}", screen.black_pixel());
}
