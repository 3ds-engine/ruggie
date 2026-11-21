use std::error::Error;

use crate::core::Core;
use crate::lua::file::*;
use mlua::Lua;
use ruggie_lib::input::buttons::Button;
use ruggie_lib::prelude::*;

pub fn render_cleon(mut rl: RuggieLib) {
    const CLEON_FRONT_DEPTH: f32 = 5.0;
    const CLEON_BACK_DEPTH: f32 = -5.0;

    let Some(sprite_sheet) = SpriteSheet::new("romfs:/gfx/sprite.t3x") else {
        return;
    };

    let Some(mut cleon_sprite) = sprite_sheet.get_sprite(0usize) else {
        return;
    };

    let x_pos = (rl.screen_witdh() / 2) as f32;
    let y_pos = (rl.screen_height() / 2) as f32;

    while rl.is_running() {
        rl.wait_for_vblank();
        let front_cleon_depth = CLEON_FRONT_DEPTH * rl.get_3d_slider_state();
        let back_cleon_depth = CLEON_BACK_DEPTH * rl.get_3d_slider_state();
        rl.begin_drawing();

        rl.clear_screen(Color::new(0, 255, 255, 255));
        //back_cleon
        rl.draw_sprite(&mut cleon_sprite, -back_cleon_depth + 10.0, 10.0);

        //front_cleon
        rl.draw_sprite(&mut cleon_sprite, -front_cleon_depth + x_pos, y_pos);

        rl.draw_top_right();
        rl.clear_screen(Color::new(0, 255, 255, 255));
        //back cleon
        rl.draw_sprite(&mut cleon_sprite, back_cleon_depth + 10.0, 10.0);

        //front cleon
        rl.draw_sprite(&mut cleon_sprite, front_cleon_depth + x_pos, y_pos);

        rl.draw_bottom();
        rl.clear_screen(Color::new(0, 255, 255, 255));
        rl.draw_sprite(&mut cleon_sprite, 1.0, 0.0);

        rl.end_drawing();
    }
}

pub fn render_rectangle(mut rl: RuggieLib) {
    while rl.is_running() {
        rl.wait_for_vblank();

        rl.begin_drawing();
        rl.clear_screen(Color::new(0, 255, 255, 255));
        rl.draw_rectangle(0.0, 0.0, 100.0, 100.0, Color::new(255, 0, 0, 255));
        rl.end_drawing();
    }
}

pub fn render_shapes(mut rl: RuggieLib) {
    const WHITE: Color = Color::new(255, 255, 255, 255);
    const RED: Color = Color::new(255, 0, 0, 255);
    const GREEN: Color = Color::new(0, 255, 0, 255);
    const BLUE: Color = Color::new(0, 0, 255, 255);
    const YELLOW: Color = Color::new(255, 255, 0, 255);
    const BLACK: Color = Color::new(0, 0, 0, 255);

    const OUTLINE: u16 = 5;
    const APPROX_SIDES: u16 = 20;

    while rl.is_running() {
        rl.wait_for_vblank();

        rl.begin_drawing();
        rl.clear_screen(WHITE);

        let square_side = 100.0;
        rl.draw_rectangle(0.0, 0.0, square_side, square_side, RED);
        rl.draw_rectangle_outline(0.0, 0.0, square_side, square_side, OUTLINE, BLACK);

        let circle_radius = 50.0;
        rl.draw_circle(
            rl.screen_witdh() as f32 - circle_radius,
            circle_radius,
            circle_radius,
            BLUE,
        );
        rl.draw_circle_outline(
            rl.screen_witdh() as f32 - circle_radius,
            circle_radius,
            circle_radius,
            APPROX_SIDES,
            OUTLINE,
            BLACK,
        );

        rl.draw_bottom();
        rl.clear_screen(WHITE);

        let ellipse_rx = 50.0;
        let ellipse_ry = 25.0;
        rl.draw_ellipse(ellipse_rx, ellipse_ry, ellipse_rx, ellipse_ry, GREEN);
        rl.draw_ellipse_outline(
            ellipse_rx,
            ellipse_ry,
            ellipse_rx,
            ellipse_ry,
            APPROX_SIDES,
            OUTLINE,
            BLACK,
        );

        let triangle_witdth = 100.0;
        let triangle_height = 75.0;
        let (x0, y0) = (rl.screen_witdh() as f32 - triangle_witdth, triangle_height);
        let (x1, y1) = (rl.screen_witdh() as f32 - triangle_witdth / 2.0, 0.0);
        let (x2, y2) = (rl.screen_witdh() as f32, triangle_height);
        rl.draw_triangle(x0, y0, x1, y1, x2, y2, YELLOW);
        rl.draw_triangle_outline(x0, y0, x1, y1, x2, y2, OUTLINE, BLACK);

        rl.end_drawing();
    }
}

pub fn rotating_cleon(rl: RuggieLib) {
    let Ok(mut rl) = rl.with([Feature::Console(BOTTOM_SCREEN_INDEX)]) else {
        return;
    }; // Enable console

    let Some(spritesheet) = SpriteSheet::new("romfs:/gfx/sprite.t3x") else {
        return;
    };

    let Some(mut cleon_sprite) = spritesheet.get_sprite(0) else {
        return;
    };

    cleon_sprite.set_pivot(0.5, 0.5); // Set position to center

    let (w, h) = cleon_sprite.get_size();
    println!("Before:");
    println!("Width: {w}, height: {h}");

    cleon_sprite.set_scale(0.5, 0.5);

    let (w, h) = cleon_sprite.get_size();
    println!("After:");
    println!("Width: {w}, height: {h}");

    let angle_diff = 1.0; // Degrees

    //rl.play_streamed_sound("romfs:/audio/test.mp3");

    while rl.is_running() {
        let dt = rl.time.since_last_check().as_secs_f32();

        println!("DeltaTime: {dt}");
        rl.wait_for_vblank();

        {
            rl.begin_drawing();
            rl.clear_screen(Color::new(255, 255, 255, 255));

            rl.draw_sprite(
                &mut cleon_sprite,
                rl.screen_witdh() as f32 / 2.0,
                rl.screen_height() as f32 / 2.0,
            );

            rl.end_drawing();
        }

        cleon_sprite.rotate_degrees(angle_diff);
    }
}

pub fn input_example(rl: RuggieLib) {
    let Ok(mut rl) = rl.with([Feature::Console(TOP_SCREEN_INDEX)]) else {
        return;
    }; // Enable console

    while rl.is_running() {
        rl.wait_for_vblank();
        rl.clear_console();
        rl.scan_input();

        println!(
            "CirclePad direction: {}, {}",
            rl.circlepad_direction().0,
            rl.circlepad_direction().1
        );

        println!("A pressed: {}", rl.button_held(Button::A));
        println!("B pressed: {}", rl.button_held(Button::B));
        println!("Y pressed: {}", rl.button_held(Button::Y));
        println!("X pressed: {}", rl.button_held(Button::X));

        println!("Start pressed: {}", rl.button_held(Button::Start));
        println!("Select pressed: {}", rl.button_held(Button::Select));

        println!("Dpad right pressed: {}", rl.button_held(Button::DPadRight));
        println!("Dpad up pressed: {}", rl.button_held(Button::DPadUp));
        println!("Dpad left pressed: {}", rl.button_held(Button::DPadLeft));
        println!("Dpad down pressed: {}", rl.button_held(Button::DPadDown));

        println!(
            "C-Stick direction: {}, {}",
            rl.c_stick_direction().0,
            rl.c_stick_direction().1
        );

        println!("L pressed: {}", rl.button_held(Button::L));
        println!("R pressed: {}", rl.button_held(Button::R));
        println!("ZL pressed: {}", rl.button_held(Button::ZL));
        println!("ZR pressed: {}", rl.button_held(Button::ZR));

        let touchscreen_info = match rl.touch_pad_position() {
            None => String::from("Touchpad pressed: False"),
            Some(value) => {
                format!("Touchpad pressed: True. Position: {}, {}", value.0, value.1)
            }
        };

        println!("{touchscreen_info}");
    }
}

pub fn audio_example(mut rl: RuggieLib) {
    let streamed_sound = rl.load_streamed_sound("romfs:/audio/test.mp3").unwrap();
    rl.play_streamed_sound(&streamed_sound).unwrap();
    while rl.is_running() {}
}

pub fn lua_example(rl: RuggieLib) {
    // Enable console
    let Ok(rl) = rl.with([Feature::Console(TOP_SCREEN_INDEX)]) else {
        return;
    };

    let mut lua = Lua::new();

    //execute_file(&mut lua, "romfs:/lua/HelloRuggie.lua");
    //execute_all_files_in_directory(&mut lua, "romfs:/lua/test/");
    let four: f64 = get_value_from_file(&mut lua, "romfs:/lua/SumTwoPlusTwo.lua");

    println!("four is: {}", four);

    while rl.is_running() {}
}

pub fn entry_point_example(rl: RuggieLib) -> Result<(), Box<dyn Error>> {
    let Ok(rl) = rl.with([Feature::Console(BOTTOM_SCREEN_INDEX)]) else {
        return Ok(());
    };

    let mut core = Core::new(rl)?;
    core.load_entry_point()?;

    while core.is_running() {
        core.render();
    }

    Ok(())
}
