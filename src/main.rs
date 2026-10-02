use ctru::{prelude::*, services::{romfs::RomFS, gfx}};

use ruggie_lib::{
    Feature, draw::sprite_sheet::SpriteSheet, prelude::*,
};

fn main() {
    let Ok(mut rl) = RuggieLib::new().unwrap().with([Feature::RomFS, Feature::Stereoscopic3D]) else {
        return;
    };
    
    render_cleon(rl);
    //let mut hid = Hid::new().unwrap();
    //let gfx = Gfx::new().unwrap();
    //let apt = Apt::new().unwrap();
    //let romfs = RomFS::new().unwrap();

    //render_cleon_deprecated(&apt, &mut hid, &gfx);
}


const CLEON_FRONT_DEPTH:f32 = 5.0;
const CLEON_BACK_DEPTH:f32 = -5.0;
fn render_cleon(mut rl: RuggieLib) {
    let Some(sprite_sheet) = SpriteSheet::new("romfs:/gfx/sprite.t3x") else {
        return;
    };

    let mut cleon_sprite = sprite_sheet.get_sprite(0usize);

    let x_pos = (rl.top_left_screen.width()/2) as f32;
    let y_pos = (rl.top_left_screen.height()/2) as f32;


    while(rl.is_running()) {
        rl.wait_for_vblank();
        let front_cleon_depth = CLEON_FRONT_DEPTH * rl.get_3d_slider_state();
        let back_cleon_depth = CLEON_BACK_DEPTH * rl.get_3d_slider_state();
        let mut draw = rl.start_drawing();


        draw.clear_screen(Color::new(0, 255, 255, 255));
        //back_cleon
        draw.draw_sprite(&mut cleon_sprite, -back_cleon_depth + 10.0, 10.0);

        //front_cleon
        draw.draw_sprite(&mut cleon_sprite, -front_cleon_depth + x_pos, y_pos);



        draw.draw_top_right();
        draw.clear_screen(Color::new(0, 255, 255, 255));
        //back cleon
        draw.draw_sprite(&mut cleon_sprite, back_cleon_depth + 10.0, 10.0);

        //front cleon
        draw.draw_sprite(&mut cleon_sprite, front_cleon_depth + x_pos, y_pos);

        draw.draw_bottom();
        draw.clear_screen(Color::new(0, 255, 255, 255));
        draw.draw_sprite(&mut cleon_sprite, 1.0, 0.0);
    }

}

fn render_cleon_deprecated(apt: &Apt, hid: &mut Hid, gfx: &Gfx) {
    ruggie_lib::deprecated::init();

    let top_screen = ruggie_lib::deprecated::create_top_screen();
    let (mut x, mut y) = (0.0, 0.0);

    let sprite_sheet = ruggie_lib::deprecated::create_sprite_sheet("romfs:/gfx/sprite.t3x").unwrap();
    let mut cleon_sprite = ruggie_lib::deprecated::create_sprite_from_sheet(sprite_sheet, 0);

    while apt.main_loop() {
        gfx.wait_for_vblank();

        ruggie_lib::deprecated::draw_sprite(top_screen, &mut cleon_sprite, x, y);
    }

    ruggie_lib::deprecated::end();
}

fn render_rectangle(mut rl: RuggieLib) {
    while rl.is_running() {
        rl.wait_for_vblank();

        let mut draw = rl.start_drawing();
        draw.clear_screen(Color::new(0, 255, 255, 255));
        draw.draw_rectangle(0.0, 0.0, 100.0, 100.0, Color::new(255, 0, 0, 255));
    }
}
