extern crate libc;
extern crate xcb;
use std::{ptr,process};

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

// xcb_create_window (c,                             /* Connection          */
//                      XCB_COPY_FROM_PARENT,          /* depth (same as root)*/
//                      win,                           /* window Id           */
//                      screen->root,                  /* parent window       */
//                      0, 0,                          /* x, y                */
//                      150, 150,                      /* width, height       */
//                      10,                            /* border_width        */
//                      XCB_WINDOW_CLASS_INPUT_OUTPUT, /* class               */
//                      screen->root_visual,           /* visual              */
//                      0, NULL);                      /* masks, not used yet */

    let win: xcb::Window = conn.generate_id();

    xcb::create_window(&conn,
                       xcb::COPY_FROM_PARENT as u8,
                       win,
                       screen.root(),
                       0, 0,
                       150, 150,
                       10,
                       xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                       screen.root_visual(),
                       &[(0, 0)]);

    xcb::map_window(&conn, win);
    conn.flush();

    unsafe {
        libc::pause();
    }
}
