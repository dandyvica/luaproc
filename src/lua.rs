use std::fs;

use mlua::{Lua as MLua, LuaSerdeExt, Table};
use quote::ToTokens;
use serde::Serialize;
use syn::{Attribute, Expr};

// definition of all useful structs for building global variable
#[derive(Debug, Default, Serialize)]
pub(super) struct Field {
    pub(super) name: String,
    pub(super) r#type: String,

    // List of inner attributes
    pub(super) attributes: Vec<Attr>,
}

#[derive(Debug, Default, Serialize)]
pub(super) struct Variant {
    pub(super) name: String,
    // pub(super) ty: String,

    // List of inner attributes
    pub(super) attributes: Vec<Attr>,

    pub(super) discriminant: Option<String>,
    pub(super) fields: Vec<Field>,
}

#[derive(Debug, Default, Serialize)]
pub(super) struct Attr {
    pub(super) name: Option<String>,
    pub(super) inner: String,
}

#[derive(Debug, Default, Serialize)]
pub(super) struct Generics {
    pub(super) r#impl: String,
    pub(super) r#type: String,
    pub(super) r#where: String,
}

pub(super) struct Lua;

impl Lua {
    //───────────────────────────────────────────────────────────────────────────────────
    // inject Rust struct or enum as a Lua global variable
    //───────────────────────────────────────────────────────────────────────────────────
    pub(super) fn lua_set_var<T: Serialize>(
        lua: &MLua,
        globals: &Table<'_>,
        glob: &T,
        var: &str,
    ) -> mlua::Result<()> {
        let lua_var = lua.to_value(glob)?;
        globals.set(var, lua_var)
    }

    //───────────────────────────────────────────────────────────────────────────────────
    // execute Lua code from the source file
    //───────────────────────────────────────────────────────────────────────────────────
    pub(super) fn lua_exec_code(lua: &MLua, path: &str) -> mlua::Result<()> {
        // open source file
        let lua_code = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("can't open Lua source file '{path}'"));

        let res = lua.load(lua_code).exec();
        if let Err(e) = res {
            // its useful to get script name for debugging
            panic!("error in Lua script {path}: {e}")
        } else {
            res
        }
    }
}

//───────────────────────────────────────────────────────────────────────────────────
// a helper function for grabing field attributes
//───────────────────────────────────────────────────────────────────────────────────
#[allow(clippy::field_reassign_with_default)]
pub(super) fn attributes<'a, T>(value: T) -> Vec<Attr>
where
    T: IntoIterator<Item = &'a Attribute>,
{
    let mut v: Vec<Attr> = Vec::new();

    for attr in value {
        let mut at = Attr::default();

        at.name = attr.meta.path().get_ident().map(|ident| ident.to_string());

        match &attr.meta {
            syn::Meta::Path(p) => {
                at.inner = p.get_ident().map_or(String::new(), |x| x.to_string());
            }
            syn::Meta::List(l) => {
                let expr: Expr = l.parse_args().unwrap();
                at.inner = expr.to_token_stream().to_string();
            }
            syn::Meta::NameValue(n) => {
                at.inner = n.value.to_token_stream().to_string();
            }
        }

        v.push(at);
    }

    v
}

//───────────────────────────────────────────────────────────────────────────────────
// helper function to get fields and its features
//───────────────────────────────────────────────────────────────────────────────────
pub(super) fn fields<'a, T>(value: T) -> Vec<Field>
where
    T: IntoIterator<Item = &'a syn::Field>,
{
    let mut v: Vec<Field> = Vec::new();

    for f in value {
        let mut field = Field::default();

        // get field name
        let ident = f.ident.as_ref().unwrap();
        field.name = ident.to_string();

        // field type
        field.r#type = f.ty.to_token_stream().to_string();

        // get inner attributes using local fn
        field.attributes = attributes(&f.attrs);

        v.push(field);
    }

    v
}
