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
}

#[macro_export]
macro_rules! register_module {
    { $parent_mod:ident, { $($mod:ident),+ $(,)? } } => {
        $(
            $parent_mod.set(stringify!($mod), $mod)?;
        )*
    };
}
