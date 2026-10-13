use std::{fs::Metadata, path::Path};
use mlua::{Error::MemoryControlNotAvailable, Lua};


fn nothing(err: std::io::Error) -> Metadata{
    println!("Error: {}", err);
    let npath = Path::new("./mod.rs");
    npath.metadata().unwrap()
}

// TEST CODE
pub fn execute_lua_file(lua: &mut Lua, path: &str){
    
    let lua_path = Path::new(path);

    println!("Path: {:?}", lua_path.as_os_str());
    //lua.load(lua_path).exec().unwrap();
}