use mlua::{Lua, UserData};
use ruggie_lib::draw::color::Color;

pub trait LuaCustomType {
    fn lua_register(lua: &Lua) -> mlua::Result<()>;
}

pub struct LuaColor(pub Color);

impl LuaCustomType for LuaColor {
    fn lua_register(lua: &Lua) -> mlua::Result<()> {
        let t = lua.create_table()?;
        t.set(
            "new",
            lua.create_function(|_, (r, g, b, a): (u8, u8, u8, Option<u8>)| {
                Ok(LuaColor(Color::new(r, g, b, a.unwrap_or(255))))
            })?,
        )?;
        t.set(
            "hex",
            lua.create_function(|_, hex: u32| {
                let [r, g, b, a] = hex.to_be_bytes();
                Ok(LuaColor(Color::new(r, g, b, a)))
            })?,
        )?;

        lua.globals().set("Color", t)
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
