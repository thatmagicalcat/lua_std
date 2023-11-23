mod modules;
mod register;

use mlua::prelude::*;

#[mlua::lua_module]
fn std(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    let env_module = modules::env::get_module(lua)?;

    exports.set("env", env_module)?;

    Ok(exports)
}
