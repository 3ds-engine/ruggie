use ctru::prelude::*;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());


    ruggie_lib::init();
    let top_screen = ruggie_lib::create_top_screen();

    while apt.main_loop() {
        gfx.wait_for_vblank();

        ruggie_lib::draw_square(top_screen);
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
    ruggie_lib::end();
}
