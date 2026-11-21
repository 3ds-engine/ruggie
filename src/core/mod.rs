use crate::{
    core::{world::World},
    lua::file::get_value_from_file,
};

use mlua::{Function, Lua, Table};

pub mod world;

pub struct Core {
    lua: Lua,
    world: Option<World>,
}

impl Core {
    pub fn new() -> mlua::Result<Self> {
        let lua = Lua::new();
        Ok(Self {
            lua,
            world: None,
        })
    }

    pub fn load_entry_point(&mut self) -> mlua::Result<()> {
        let scene: Table = get_value_from_file(&mut self.lua, "romfs:/game/main.lua");

        if let Ok(init) = scene.get::<Function>("init") {
            init.call::<()>(self.lua.create_table()?)?;
        }

        self.world = Some(World::new(scene));

        Ok(())
    }
}
