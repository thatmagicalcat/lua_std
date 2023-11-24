use std::path::PathBuf;

use mlua::prelude::*;

pub struct DirEntry {
    path: Option<PathBuf>,
}

impl DirEntry {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self {
            path: Some(PathBuf::from(path.into())),
        }
    }

    pub fn init() -> Self {
        Self { path: None }
    }
}

impl LuaUserData for DirEntry {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("path", |_, this| {
            Ok(this.path.as_ref().unwrap().to_str().unwrap().to_string())
        });

        fields.add_field_method_set("path", |_, this, val: String| {
            *this.path.as_mut().unwrap() = PathBuf::from(val);
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_dir", |_, this, ()| {
            Ok(this.path.as_ref().unwrap().is_dir())
        });

        methods.add_method("open", |_, _, path: String| Ok(DirEntry::new(path)));

        methods.add_method("file_size", |_, this, ()| {
            Ok(this.path.as_ref().unwrap().metadata().into_lua_err()?.len())
        });

        methods.add_method("file_name", |_, this, ()| {
            Ok(this
                .path
                .as_ref()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string())
        });

        methods.add_method("is_readonly", |_, this, ()| {
            Ok(this
                .path
                .as_ref()
                .unwrap()
                .metadata()
                .into_lua_err()?
                .permissions()
                .readonly())
        });

        methods.add_method("modified", |_, this, ()| {
            Ok(this
                .path
                .as_ref()
                .unwrap()
                .metadata()
                .unwrap()
                .modified()
                .into_lua_err()?
                .elapsed()
                .into_lua_err()?
                .as_secs())
        });

        methods.add_method("created", |_, this, ()| {
            Ok(this
                .path
                .as_ref()
                .unwrap()
                .metadata()
                .into_lua_err()?
                .created()
                .into_lua_err()?
                .elapsed()
                .into_lua_err()?
                .as_secs())
        });

        methods.add_method("is_symlink", |_, this, ()| {
            Ok(this
                .path
                .as_ref()
                .unwrap()
                .metadata()
                .into_lua_err()?
                .is_symlink())
        });
    }
}
