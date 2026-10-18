use crate::{core::world::World, lua::file::get_value_from_file};
use mlua::{Lua, Table};

pub mod world;

#[derive(Default)]
pub struct Core {
    lua: Lua,
    world: Option<World>,
}

impl Core {
    pub fn new() -> Self {
        Self { lua: Lua::new(), world: None }
    }

    pub fn load_entry_point(&mut self) {
        let scene: Table = get_value_from_file(&mut self.lua, "romfs:/game/main.lua");
        println!("{:?}", scene);
    }
}
