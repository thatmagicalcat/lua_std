use mlua::prelude::*;

pub fn stop(lua: &Lua, _: ()) -> LuaResult<()> {
    lua.gc_stop();
    Ok(())
}

pub fn restart(lua: &Lua, _: ()) -> LuaResult<()> {
    lua.gc_stop();
    Ok(())
}

pub fn collect(lua: &Lua, _: ()) -> LuaResult<()> {
    lua.gc_collect()?;
    lua.gc_collect()?;

    Ok(())
}

pub fn step_kbytes(lua: &Lua, kbytes: i32) -> LuaResult<impl IntoLua> {
    lua.gc_step_kbytes(kbytes)
}

pub fn set_pause(lua: &Lua, pause: i32) -> LuaResult<impl IntoLua> {
    Ok(lua.gc_set_pause(pause))
}

pub fn set_multiplier(lua: &Lua, step_multiplier: i32) -> LuaResult<impl IntoLua> {
    Ok(lua.gc_set_step_multiplier(step_multiplier))
}

pub fn incremental(
    lua: &Lua,
    (pause, step_multipler, step_size): (i32, i32, i32),
) -> LuaResult<impl IntoLua> {
    let mode = match lua.gc_inc(pause, step_multipler, step_size) {
        LuaGCMode::Incremental => "incremental",
        LuaGCMode::Generational => "generational",
    };

    Ok(mode)
}

pub fn generational(
    lua: &Lua,
    (minor_multiplier, major_multiplier): (i32, i32),
) -> LuaResult<impl IntoLua> {
    let mode = match lua.gc_gen(minor_multiplier, major_multiplier) {
        LuaGCMode::Incremental => "incremental",
        LuaGCMode::Generational => "generational",
    };

    Ok(mode)
}

pub fn step(lua: &Lua, _: ()) -> LuaResult<impl IntoLua> {
    lua.gc_step()
}
