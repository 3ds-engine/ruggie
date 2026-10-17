use crate::lua::file::get_value_from_file;
use mlua::{Lua, Table};

pub struct Core {
    lua: Lua,
}

impl Core {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }

    pub fn load_entry_point(&mut self) {
        let scene: Table = get_value_from_file(&mut self.lua, "romfs:/game/main.lua");
        println!("{:?}", scene);
    }
}
