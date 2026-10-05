use ctru::{
    prelude::*,
    services::{gfx, romfs::RomFS},
};

use ruggie_lib::{Feature, draw::sprite_sheet::SpriteSheet, prelude::*};

fn main() {
    let Ok(rl) = RuggieLib::new()
        .unwrap()
        .with([Feature::RomFS, Feature::Stereoscopic3D])
    else {
        return;
    };

    render_shapes(rl);
}

fn render_cleon(mut rl: RuggieLib) {
    const CLEON_FRONT_DEPTH: f32 = 5.0;
    const CLEON_BACK_DEPTH: f32 = -5.0;

    let Some(sprite_sheet) = SpriteSheet::new("romfs:/gfx/sprite.t3x") else {
        return;
    };

    let mut cleon_sprite = sprite_sheet.get_sprite(0usize);

    let x_pos = (rl.top_left_screen.width() / 2) as f32;
    let y_pos = (rl.top_left_screen.height() / 2) as f32;

    while (rl.is_running()) {
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

fn render_rectangle(mut rl: RuggieLib) {
    while rl.is_running() {
        rl.wait_for_vblank();

        let mut draw = rl.start_drawing();
        draw.clear_screen(Color::new(0, 255, 255, 255));
        draw.draw_rectangle(0.0, 0.0, 100.0, 100.0, Color::new(255, 0, 0, 255));
    }
}

fn render_shapes(mut rl: RuggieLib) {
    const WHITE: Color = Color::new(255, 255, 255, 255);
    const RED: Color = Color::new(255, 0, 0, 255);
    const GREEN: Color = Color::new(0, 255, 0, 255);
    const BLUE: Color = Color::new(0, 0, 255, 255);
    const YELLOW: Color = Color::new(255, 255, 0, 255);
    const BLACK: Color = Color::new(0, 0, 0, 255);

    const OUTLINE : u32 = 5;

    while rl.is_running() {
        rl.wait_for_vblank();

        let mut draw = rl.start_drawing();
        draw.clear_screen(WHITE);

        let square_side = 100.0;
        draw.draw_rectangle(0.0, 0.0, square_side, square_side, RED);
        draw.draw_rectangle_outline(0.0, 0.0, square_side, square_side, OUTLINE, BLACK);

        let circle_radius = 50.0;
        draw.draw_circle(
            draw.screen_witdh() as f32 - circle_radius,
            circle_radius,
            circle_radius,
            BLUE,
        );

        draw.draw_bottom();
        draw.clear_screen(WHITE);

        let ellipse_rx = 50.0;
        let ellipse_ry = 25.0;
        draw.draw_ellipse(ellipse_rx, ellipse_ry, ellipse_rx, ellipse_ry, GREEN);

        let triangle_witdth = 100.0;
        let triangle_height = 75.0;
        let (x0, y0) = (
            draw.screen_witdh() as f32 - triangle_witdth,
            triangle_height,
        );
        let (x1, y1) = (draw.screen_witdh() as f32 - triangle_witdth / 2.0, 0.0);
        let (x2, y2) = (draw.screen_witdh() as f32, triangle_height);
        draw.draw_triangle(x0, y0, x1, y1, x2, y2, YELLOW);
        draw.draw_triangle_outline(x0, y0, x1, y1, x2, y2, OUTLINE, BLACK);

    }
}
