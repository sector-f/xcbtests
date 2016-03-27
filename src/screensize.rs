// Output screen size in WIDTHxHEIGHT format

extern crate xcb;
use xcb::base;
use xcb::xproto;

fn main() {
    let (connection, screen_num) = base::Connection::connect();

    let setup: xproto::Setup = connection.get_setup();

    // let roots_len = unsafe { (*setup.ptr).roots_len };
    let screen_iterator = setup.roots();

    let data = screen_iterator.data;
    let width = unsafe { ((*data).width_in_pixels) };
    let height = unsafe { ((*data).height_in_pixels) };

    println!("{}x{}", width, height);
}
