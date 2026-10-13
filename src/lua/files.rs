use std::fs::*;
use mlua::Lua;

// TEST CODE
pub fn execute_lua_file(lua: &mut Lua, path: &str){
    
    let lua_file_content = read(path).unwrap();
    lua.load(lua_file_content).exec().unwrap();
}