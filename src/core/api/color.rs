use mlua::{Lua, UserData};
use ruggie_lib::prelude::Color;

use super::LuaApi;

#[derive(Debug, Clone, Copy)]
pub struct LuaColor(pub Color);

impl LuaApi for LuaColor {
    fn api_register(lua: &Lua) -> mlua::Result<()> {
        // Create table
        let color = lua.create_table()?;

        // Factories
        color.set(
            "new",
            lua.create_function(|_, (r, g, b, a): (u8, u8, u8, Option<u8>)| {
                Ok(LuaColor(Color::new(r, g, b, a.unwrap_or(255u8))))
            })?,
        )?;
        color.set(
            "hex",
            lua.create_function(|_, hex: u32| Ok(LuaColor(Color::from_hex(hex))))?,
        )?;

        // Constants
        color.set("RED", LuaColor(Color::red()))?;
        color.set("GREEN", LuaColor(Color::green()))?;
        color.set("BLUE", LuaColor(Color::blue()))?;
        color.set("CYAN", LuaColor(Color::cyan()))?;
        color.set("MAGENTA", LuaColor(Color::magenta()))?;
        color.set("YELLOW", LuaColor(Color::yellow()))?;
        color.set("PINK", LuaColor(Color::pink()))?;
        color.set("ORANGE", LuaColor(Color::orange()))?;
        color.set("BROWN", LuaColor(Color::brown()))?;
        color.set("WHITE", LuaColor(Color::white()))?;
        color.set("BLACK", LuaColor(Color::black()))?;
        color.set("GRAY", LuaColor(Color::gray()))?;

        // Register table
        lua.globals().set("Color", color)
    }
}

impl UserData for LuaColor {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        // Getters
        fields.add_field_method_get("r", |_, this| Ok(this.0.r));
        fields.add_field_method_get("g", |_, this| Ok(this.0.g));
        fields.add_field_method_get("b", |_, this| Ok(this.0.b));
        fields.add_field_method_get("a", |_, this| Ok(this.0.a));

        // Setters
        fields.add_field_method_set("r", |_, this, value: u8| {
            this.0.r = value;
            Ok(())
        });
        fields.add_field_method_set("g", |_, this, value: u8| {
            this.0.g = value;
            Ok(())
        });
        fields.add_field_method_set("b", |_, this, value: u8| {
            this.0.b = value;
            Ok(())
        });
        fields.add_field_method_set("a", |_, this, value: u8| {
            this.0.a = value;
            Ok(())
        });
    }
}
