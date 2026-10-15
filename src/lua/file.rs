use std::fs::*;
use mlua::{FromLuaMulti, Lua};

pub fn execute_file(lua: &mut Lua, path: &str){
    
    let lua_file_content = read(path).unwrap();
    lua.load(lua_file_content).exec().unwrap();
}

pub fn execute_all_files_in_directory(lua: &mut Lua, path: &str){

    let lua_directory = read_dir(path).unwrap();

    for file in lua_directory {

        let curr_file = file.unwrap();
        let curr_file_path = curr_file.path();

        if(curr_file.metadata().unwrap().is_file() && 
            curr_file_path.extension().unwrap() == "lua"){
            lua.load(curr_file_path).exec().unwrap();
        }
    }

}

pub fn get_value_from_file<T: FromLuaMulti>(lua: &mut Lua, path: &str) -> T {

    let lua_file_content = read(path).unwrap();
    let result: T = lua.load(lua_file_content).eval().unwrap();

    return result;
}