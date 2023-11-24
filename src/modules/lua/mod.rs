use crate::*;
use mlua::prelude::*;

pub fn get_module(lua: &Lua) -> LuaResult<LuaTable> {
    let lua_mod = lua.create_table()?;

    use gc::*;
    let gc_mod = lua.create_table()?;
    register_functions!(
        lua,
        mod gc_mod {
            step,
            restart,
            collect,
            step_kbytes,
            set_pause,
            set_multiplier,
            incremental,
            generational,
            stop
        }
    );

    use functions::*;
    register_functions!(
        lua,
        mod lua_mod {
            get_used_memory,
            set_memory_limit
        }
    );

    lua_mod.set("gc", gc_mod)?;

    Ok(lua_mod)
}

mod functions;
mod gc;
