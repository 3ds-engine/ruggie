use std::{cell::RefCell, rc::Rc};

use crate::{
    core::{api::Api, world::World},
    lua::file::get_value_from_file,
};

use mlua::{Function, Lua, Table};
use ruggie_lib::prelude::RuggieLib;

pub mod api;
pub mod world;

/// Shared handle to the engine instance. API modules can hold a clone of this to expose their
/// functionality without owning the `RuggieLib` instance that core may still use
pub type SharedRuggieLib = Rc<RefCell<RuggieLib>>;

pub struct Core {
    lua: Lua,
    api: Api,
    rl: SharedRuggieLib,
    world: Option<World>,
}

impl Core {
    /// # Errors
    /// Fails if any Lua API can't be registered.
    pub fn new(rl: RuggieLib) -> mlua::Result<Self> {
        let lua = Lua::new();
        let shared_rl = Rc::new(RefCell::new(rl));
        let api = Api::new(shared_rl.clone(), &lua)?;

        Ok(Self {
            lua,
            api,
            rl: shared_rl,
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
        self.rl.borrow().wait_for_vblank();
        self.rl.borrow().begin_drawing();

        if let Some(world) = &self.world {
            world.render(self.api.renderer_userdata());
        }

        self.rl.borrow().end_drawing();
    }

    pub fn is_running(&self) -> bool {
        self.rl.borrow().is_running()
    }
}
