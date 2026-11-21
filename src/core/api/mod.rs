use mlua::{AnyUserData, Lua};

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
    renderer: AnyUserData, // This is actually a Renderer2DApi
}

impl Api {
    /// # Errors
    /// Fails if any Lua API can't be registered.
    pub fn new(rl: SharedRuggieLib, lua: &Lua) -> mlua::Result<Self> {
        color::LuaColor::api_register(lua)?;

        let renderer_api = Renderer2DApi::new(rl);
        let renderer = lua.create_userdata(renderer_api)?;

        Ok(Self {
            renderer,
        })
    }

    pub const fn renderer_userdata(&self) -> &AnyUserData{
        &self.renderer
    }
}
