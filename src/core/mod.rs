use crate::{core::{renderer2d::RendererApi, world::World}, lua::file::get_value_from_file};
use mlua::{Function, Lua, Table};

pub mod world;
pub mod renderer2d;

pub struct Core {
    lua: Lua,
    world: Option<World>,
}

impl Core {
    pub fn new() -> Self {
        Self { lua: Lua::new(), world: None}
    }

    pub fn load_entry_point(&mut self) {
        let scene: Table = get_value_from_file(&mut self.lua, "romfs:/game/main.lua");
        let init : Function = scene.get("init").unwrap();
        init.call::<()>(self.lua.create_table().unwrap());
    }
}
