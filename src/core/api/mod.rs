use mlua::Lua;

use crate::core::{SharedRuggieLib, api::renderer::Renderer2DApi};

pub mod color;
pub mod renderer;


/// Implemented by every Lua API object. `api_register` sets global type like tables into the lua 
/// state once
pub trait LuaApi {
    fn api_register(lua: &Lua) -> mlua::Result<()>;
}

/// Entry point for everything Lua-related that exposes `RuggieLib`.
pub struct Api {
    renderer: Renderer2DApi,
}

impl Api {
    pub fn new(rl: SharedRuggieLib, lua: &Lua) -> mlua::Result<Self> {
        color::LuaColor::api_register(lua)?;

        let renderer = Renderer2DApi::new(rl);
        Ok(Self { renderer })
    }

    pub fn renderer(&self) -> &Renderer2DApi {
        &self.renderer
    }
}
