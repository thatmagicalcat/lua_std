use super::types::DirEntry;
use std::{fs, io::Read};

use mlua::prelude::*;

pub fn read_dir(_: &Lua, path: String) -> LuaResult<impl IntoLua> {
    let mut entries = vec![];

    for entry in fs::read_dir(path).into_lua_err()? {
        entries.push(DirEntry::new(entry?.path()));
    }

    Ok(entries)
}

pub fn exists(_: &Lua, path: String) -> LuaResult<impl IntoLua> {
    Ok(std::path::Path::new(&path).exists())
}

pub fn read_file(_: &Lua, path: String) -> LuaResult<impl IntoLua> {
    let mut file = fs::File::open(path)?;
    let mut data = Vec::with_capacity(file.metadata().unwrap().len() as _);
    let _ = file.read_to_end(&mut data)?;

    Ok(data)
}
