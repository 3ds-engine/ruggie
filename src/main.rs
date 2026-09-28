use ctru::{prelude::*, services::ir_user::IrDeviceId::CirclePadPro};

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let console = Console::new(gfx.bottom_screen.borrow_mut());

    let romfs = ctru::services::romfs::RomFS::new().unwrap();

    ruggie_lib::init();
    render_moving_rectangle(apt, hid, &gfx);
    ruggie_lib::end();
}

fn render_cleon(apt: &Apt, hid: &mut Hid, gfx: &Gfx) {
    let top_screen = ruggie_lib::create_top_screen();
    let (mut x, mut y) = (0.0, 0.0);

    let mut sprite_sheet = ruggie_lib::create_sprite_sheet("romfs:/gfx/spritesheet.t3x").unwrap();
    let mut cleon_sprite = ruggie_lib::create_sprite_from_sheet(sprite_sheet, 0);

    while apt.main_loop() {
        gfx.wait_for_vblank();
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        ruggie_lib::draw_sprite(top_screen, &mut cleon_sprite, x, y);
        x += 1.0;
        y += 1.0;
    }
}

fn render_moving_rectangle(apt: Apt, mut hid: Hid, gfx: &Gfx) {
    let (mut x, mut y) = (0.0, 0.0);
    let top_screen = ruggie_lib::create_top_screen();
    while apt.main_loop() {
        gfx.wait_for_vblank();

        let mut x_dir = hid.circlepad_position().0 as f32 / 156.0;
        let mut y_dir = -hid.circlepad_position().1 as f32 / 156.0;

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        if hid.keys_held().contains(KeyPad::DPAD_LEFT){
            x_dir = -1.0;
        }

        if hid.keys_held().contains(KeyPad::DPAD_RIGHT){
            x_dir = 1.0;
        }

        if hid.keys_held().contains(KeyPad::DPAD_UP){
            y_dir = -1.0;
        }


        if hid.keys_held().contains(KeyPad::DPAD_DOWN){
            y_dir = 1.0;
        }

        x += x_dir;

        y+= y_dir;


        ruggie_lib::draw_square(top_screen, x, y);

        let c_x = hid.circlepad_position().0;
        let c_y = hid.circlepad_position().1;

        print!("\x1B[2J");
        println!("\x1B[2JDir_x: {x_dir}\nDir_y: {y_dir}\nCircle_pad: {c_x} {c_y}");
    }
}

