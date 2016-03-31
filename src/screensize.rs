// Output screen sizes in WIDTHxHEIGHT format

extern crate xcb;
use xcb::base;
use xcb::xproto;

fn main() {
    let (connection, screen_num) = base::Connection::connect();

    let setup: xproto::Setup = connection.get_setup();

    let screen_iterator = setup.roots();

    for screen in screen_iterator {
        let width = screen.width_in_pixels();
        let height = screen.height_in_pixels();
        println!("{}x{}", width, height);
    }
}
