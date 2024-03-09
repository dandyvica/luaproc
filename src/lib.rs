//! This crate allows you to develop your derive procedural macros in the Lua language.
//! 
//! This is how it's achieved inside the ```luaproc``` function:
//! 
//! * build the ast as a ```DeriveInput``` struct
//! * depending on whether it's a struct or an enum you want to derive (unions not supported), a specific Rust structure is
//! built gathering all metadata like outer generics, attributes, fields, types, etc
//! * because those specific structures are ```Serialize```, using the ```mlua``` crate and the powerful ```to_value()```
//! ```mlua``` method, either the ```struct``` or ```enum``` Lua global variable is created
//! * the script is executed
//! * the ```code``` Lua global contains the Rust code which is returned by the macro as a ```TokenStream```
//! 
//! ## Howto
//! 
//! Just add ```#[derive(LuaProc)]``` is the list of derive attributes 
//! and the ```#[luaproc("path_to_script")]``` one.
//! 
//! ## Example
//! 
//! Suppose you've got the following struct:
//! 
//! ```rust,ignore
//! #[derive(Debug, Default, LuaProc)]
//! #[luaproc("tests/point2.lua")]
//! struct Point2<T>
//! where
//!     T: Copy,
//!     T: std::ops::Add<Output = T>,
//!     T: Default,
//! {
//!     x0: T,
//!     x1: T,
//!     x2: T,
//!     x3: T,
//!     x4: T,
//!     x5: T,
//!     x6: T,
//!     x7: T,
//!     x8: T,
//!     x9: T,
//! }
//! ```
//! 
//! and you want to define a function which is summing up all fields. The ```point2.lua``` script could be
//! like this:
//! 
//! ```lua
//! -- build a table with all fields name
//! local fields = {}
//! for _,f in ipairs(struct.fields) do
//!   table.insert(fields, "pt." .. f.name)
//! end
//! local inner = table.concat(fields, "+")
//! code = string.format("pub fn add<T>(pt: &%s<T>) -> T where T: std::ops::Add<Output = T> + Copy + Default { %s }", struct.meta.ident, inner)
//! ```
//!  
//! Look at the ```tests``` directory for other examples.
//! 
//! ## Caveats
//! 
//! * only Lua 5.4 is supported
//! * if the Lua script is changed, you need to force recompilation to take it into account. For example:
//! 
//! ```bash
//! $ cargo clean -p my_pkg && cargo test
//! ```
//! 
use lua_meta::LuaMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

use mlua::Lua as MLua;

mod lua;
use lua::Lua;

mod lua_struct;
use lua_struct::*;

mod lua_enum;
use lua_enum::*;

mod lua_meta;

//───────────────────────────────────────────────────────────────────────────────────
// The proc macro itself
//───────────────────────────────────────────────────────────────────────────────────
#[proc_macro_derive(LuaProc, attributes(luaproc))]
pub fn luaproc(input: TokenStream) -> TokenStream {
    // build ast
    let ast = parse_macro_input!(input as DeriveInput);
    // let ident = ast.ident.to_string();

    // get Lua context and globals
    let lua = MLua::new();
    let globals = lua.globals();

    // get meta information like generics and outer attributes
    let lua_meta = LuaMeta::new(&ast);
    // lua_meta
    //     .lua_set_var(&lua, &globals)
    //     .unwrap_or_else(|e| panic!("Lua error: {}", e));

    // extract lua script name for #[luaproc] attr
    let lua_script = lua_meta
        .lua_script()
        .unwrap_or_else(|| panic!("the #[luaproc] attribute is not found"));
    // println!(" <{}>", lua_script.trim_matches('"'));

    match &ast.data {
        Data::Struct(ds) => {
            let mut lua_struct = LuaStruct::new(ds);
            lua_struct.meta = lua_meta;

            Lua::lua_set_var(&lua, &globals, &lua_struct, "struct")
                .unwrap_or_else(|e| panic!("Lua {}", e));
        }
        Data::Enum(de) => {
            let mut lua_enum = LuaEnum::new(de);
            lua_enum.meta = lua_meta;

            Lua::lua_set_var(&lua, &globals, &lua_enum, "enum")
                .unwrap_or_else(|e| panic!("Lua {}", e));
        }
        _ => unimplemented!("{} is not an enum", ast.ident.to_string()),
    };

    // exec lua code
    Lua::lua_exec_code(&lua, lua_script.trim_matches('"')).unwrap_or_else(|e| panic!("Lua {}", e));

    // retrieve code from Lua variable named "code" and return it as a TokenStream
    if let Ok(code) = globals.get::<_, String>("code") {
        // convert this string into a token stream to be understandable by Rust compiler
        let expr = syn::parse_str::<proc_macro2::TokenStream>(&code).unwrap_or_else(|_| {
            panic!(
                "injected code '{}' from Lua is not a well-formed Rust code",
                code
            )
        });

        //println!("{}", code);
        expr.into()
    } else {
        quote!().into()
    }
}
