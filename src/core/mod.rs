use crate::{core::{renderer2d::RendererApi, world::World}, lua::file::get_value_from_file};
use mlua::{Function, Lua, Table};
use ruggie_lib::draw::RuggieDrawHandle;

pub mod world;
pub mod renderer2d;

pub struct Core {
    lua: Lua,
    world: Option<World>,
    renderer: RendererApi,
}

impl Core {
    pub fn new() -> mlua::Result<Self> {
        let lua = Lua::new();
        let renderer = RendererApi::new(&lua)?;
        Ok(Self { lua, world: None, renderer})
    }

    pub fn load_entry_point(&mut self) -> mlua::Result<()> {
        let scene: Table = get_value_from_file(&mut self.lua, "romfs:/game/main.lua");

        if let Ok(init) = scene.get::<Function>("init") {
            init.call::<()>(self.lua.create_table()?)?;        
        }

        self.world = Some(World::new(scene));

        Ok(())
    }

    pub fn render(&self, draw: &mut RuggieDrawHandle<'_>) {
        if let Some(world) = &self.world {
            self.renderer.frame(draw, |renderer| world.render(renderer)).unwrap();
        } else {
            println!("Can't render, sorry!")
        }
    }
}
