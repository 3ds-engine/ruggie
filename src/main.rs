use ruggie::examples::*;
use ruggie_lib::prelude::*;

fn main() {
    let Ok(rl) = RuggieLib::new() else {
        return;
    };

    let Ok(rl) = rl.with([Feature::RomFS, Feature::Stereoscopic3D]) else {
        return;
    };

    // Example here
    rotating_cleon(rl);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn try_get_valid_sprite() {
        let sprite_sheet = SpriteSheet::new("romfs:/gfx/sprite.t3x").unwrap();
        let sprite = sprite_sheet.get_sprite(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn try_get_invalid_sprite() {
        let sprite_sheet = SpriteSheet::new("romfs:/gfx/sprite.t3x").unwrap();
        let sprite = sprite_sheet.get_sprite(1).unwrap(); // Panic happens here
    }
}
