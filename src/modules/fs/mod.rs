use crate::*;
use mlua::prelude::*;

pub fn get_module(lua: &Lua) -> LuaResult<LuaTable> {
    let fs_mod = lua.create_table()?;

    use functions::*;
    register_functions!(
        lua,
        mod fs_mod {
            read_dir,
            exists,
            read_file
        }
    );

    use types::DirEntry;
    register_data!(
        mod fs_mod {
            "DirEntry" => DirEntry::init()
        }
    );

    Ok(fs_mod)
}

mod functions;
mod types;