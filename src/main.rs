use ruggie::examples::*;
use ruggie_lib::prelude::*;

fn main() {
    let rl = RuggieLib::new()
        .unwrap()
        .with([Feature::RomFS, Feature::Stereoscopic3D])
        .unwrap();

    // Example here
    entry_point_example(rl);
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
