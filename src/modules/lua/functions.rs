use mlua::prelude::*;

// use thiserror::Error;
// #[derive(Debug, Error)]
// pub enum MyError {
//     #[error("invalid trigger: {0}, valid triggers are: 'on_call', 'on_return' and 'each_line'")]
//     InvalidTrigger(String),
// }

pub fn get_used_memory(lua: &Lua, _: ()) -> LuaResult<impl IntoLua> {
    Ok(lua.used_memory())
}

pub fn set_memory_limit(lua: &Lua, limit: usize) -> LuaResult<impl IntoLua> {
    lua.set_memory_limit(limit)
}
