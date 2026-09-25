use ctru::prelude::*;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    ruggie_lib::init();
    let top_screen = ruggie_lib::create_top_screen();

    let (mut x, mut y) = (0.0, 0.0);
    while apt.main_loop() {
        gfx.wait_for_vblank();
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        ruggie_lib::draw_square(top_screen, x, y);
        x += 1.0;
        y += 1.0;
    }
    ruggie_lib::end();
}
