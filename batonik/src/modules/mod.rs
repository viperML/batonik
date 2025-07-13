macro_rules! mod_export {
    ($name:ident) => {
        mod $name;
        pub use crate::modules::$name::*;
    };
}

mod_export!(git);
mod_export!(env);
mod_export!(directory);