use mlua::Lua;

pub mod color;

/// Implemented by every Lua API object. `api_register` sets global type like tables into the lua 
/// state once
pub trait LuaApi {
    fn api_register(lua: &Lua) -> mlua::Result<()>;
}

/// Entry point for everything Lua-related that exposes `RuggieLib`.
#[derive(Debug, Default, Clone, Copy)]
pub struct Api;

impl Api {
    pub fn new(lua: &Lua) -> mlua::Result<Self> {
        color::LuaColor::api_register(lua)?;
        Ok(Self)
    }
}
