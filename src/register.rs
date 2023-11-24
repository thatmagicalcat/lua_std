#[macro_export]
macro_rules! register_functions {
    { $lua:ident, mod $mod:ident { $($fn_name:ident),+ $(,)? }} => {
        $(
            $mod.set(stringify!($fn_name), $lua.create_function($fn_name)?)?;
        )*
    };
}

#[macro_export]
macro_rules! register_data {
    { mod $mod:ident { $($name:ident),+ $(,)? }} => {
        $(
            $mod.set(stringify!($name), $name)?;
        )*
    };

    { mod $mod:ident { $($name:expr=> $val:expr),+ $(,)? }} => {
        $(
            $mod.set($name, $val)?;
        )*
    };
}

#[macro_export]
macro_rules! register_module {
    { mod $parent_mod:ident { $($mod_name:expr=> $val:expr),+ $(,)? } } => {
        $(
            $parent_mod.set($mod_name, $val)?;
        )*
    };
}
