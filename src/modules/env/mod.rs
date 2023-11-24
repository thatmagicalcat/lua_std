use crate::*;
use mlua::prelude::*;

pub fn get_module(lua: &Lua) -> LuaResult<LuaTable> {
    let env_mod = lua.create_table()?;

    use functions::*;
    register_functions!(
        lua,
        mod env_mod {
            args, // supplied command-line arguments
            get_var, // get a specific env variable
            set_var, // sets an env var
            remove_var, // removes a env variable
            current_dir, // directory in which program is ran
            interpreter_path, // path to lua interpreter
            vars, // get all the env variables
            temp_dir, // returns the path to temporary directory
        }
    );

    use consts::*;
    let env_consts = lua.create_table()?;
    register_data!(
        mod env_consts {
            ARCH,
            OS,
            OS_FAMILY
        }
    );

    env_mod.set("consts", env_consts)?;

    Ok(env_mod)
}

mod consts;
mod functions;
