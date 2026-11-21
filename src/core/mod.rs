use crate::{
    core::{api::Api, world::World},
    lua::file::get_value_from_file,
};

use mlua::{Function, Lua, Table};
use ruggie_lib::prelude::RuggieLib;

pub mod api;
pub mod world;

pub struct Core {
    lua: Lua,
    api: Api,
    rl: RuggieLib,
    world: Option<World>,
}

impl Core {
    /// # Errors
    /// Fails if any Lua API can't be registered.
    pub fn new(rl: RuggieLib) -> mlua::Result<Self> {
        let lua = Lua::new();
        let api = Api::new(&lua)?;

        Ok(Self {
            lua,
            api,
            rl,
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

    pub fn render(&mut self) {
        self.rl.wait_for_vblank();
        self.rl.begin_drawing();
        todo!("Render the current scene");
        self.rl.end_drawing();
    }

    pub fn is_running(&self) -> bool {
        self.rl.is_running()
    }
}
