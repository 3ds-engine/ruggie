use std::fs::File;

use ctru::prelude::*;

const WIN_WIDTH : f32 = 400.0;
const WIN_HEIGHT : f32 = 240.0;

struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let console = Console::new(gfx.bottom_screen.borrow_mut());

    let romfs = ctru::services::romfs::RomFS::new().unwrap();

    ruggie_lib::init();
    render_cleon(&apt, &mut hid, &gfx);
    // render_moving_rectangle(&apt, &mut hid, &gfx);
    ruggie_lib::end();
}

fn render_cleon(apt: &Apt, hid: &mut Hid, gfx: &Gfx) {
    let top_screen = ruggie_lib::create_top_screen();
    let (mut x, mut y) = (0.0, 0.0);
    
    let mut sprite_sheet = ruggie_lib::create_sprite_sheet("romfs:/gfx/sprite.t3x").unwrap();
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

fn render_moving_rectangle(apt: &Apt, hid: &mut Hid, gfx: &Gfx) {
    let top_screen = ruggie_lib::create_top_screen();

    let (mut x, mut y) = (0.0, 0.0);
    let mut vel = Velocity { x: 1.0, y: 1.0, };

    while apt.main_loop() {
        gfx.wait_for_vblank();
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        ruggie_lib::draw_square(top_screen, x, y);
        x+=vel.x;
        y+=vel.y;

        update_direction(&mut vel, x, y);
    }
}

fn update_direction(vel: &mut Velocity, x: f32, y: f32) {
    if x < 0.0 || x + 50.0 > WIN_WIDTH {
        vel.x = -vel.x;
    }

    if y < 0.0 || y + 50.0 > WIN_HEIGHT {
        vel.y = -vel.y;
    }
}
