use std::{collections::HashMap, env};

use mlua::prelude::*;

pub fn args(_: &Lua, _: ()) -> LuaResult<impl IntoLua> {
    Ok(env::args().map(|i| i.into_boxed_str()).collect::<Vec<_>>())
}

pub fn get_var(_: &Lua, name: String) -> LuaResult<impl IntoLua> {
    Ok(env::var(name).into_lua_err()?)
}

pub fn current_dir(_: &Lua, _: ()) -> LuaResult<impl IntoLua> {
    Ok(env::current_dir()?.to_str().unwrap().to_owned())
}

pub fn interpreter_path(_: &Lua, _: ()) -> LuaResult<impl IntoLua> {
    Ok(env::current_exe()?.to_str().unwrap().to_owned())
}

pub fn set_var(_: &Lua, (key, value): (String, String)) -> LuaResult<()> {
    env::set_var(key, value);
    Ok(())
}

pub fn remove_var(_: &Lua, key: String) -> LuaResult<()> {
    env::remove_var(key);
    Ok(())
}

pub fn temp_dir(_: &Lua, _: ()) -> LuaResult<impl IntoLua> {
    Ok(env::temp_dir().to_str().unwrap().to_owned())
}

pub fn vars(_: &Lua, _: ()) -> LuaResult<impl IntoLua> {
    Ok(HashMap::<String, String>::from_iter(
        env::vars().into_iter(),
    ))
}
