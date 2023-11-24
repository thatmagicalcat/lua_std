mod modules;
mod register;

use mlua::prelude::*;

#[mlua::lua_module]
fn std(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    register_module!(
        mod exports {
            "env" => modules::env::get_module(lua)?,
            "lua" => modules::lua::get_module(lua)?,
            "fs" => modules::fs::get_module(lua)?,
        }
    );

    Ok(exports)
}
