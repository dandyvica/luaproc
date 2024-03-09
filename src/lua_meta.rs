use quote::ToTokens;
use syn::DeriveInput;

use serde::Serialize;

use crate::lua::{attributes, Attr, Generics};

// structure holding attributes and generics for the whole struct or enum
#[derive(Debug, Default, Serialize)]
pub(super) struct LuaMeta {
    // name of struct or enum
    ident: String,

    // List of outer attributes
    attributes: Vec<Attr>,

    // generics
    generics: Generics,
}

impl LuaMeta {
    #[allow(clippy::field_reassign_with_default)]
    pub(super) fn new(di: &DeriveInput) -> Self {
        let mut lua_meta = Self::default();

        lua_meta.ident = di.ident.to_string();

        // manage generics
        let (impl_generics, ty_generics, where_clause) = di.generics.split_for_impl();

        lua_meta.generics.r#impl = impl_generics.to_token_stream().to_string();
        lua_meta.generics.r#type = ty_generics.to_token_stream().to_string();
        lua_meta.generics.r#where = where_clause.to_token_stream().to_string();

        // get outer attributes
        lua_meta.attributes = attributes(&di.attrs);

        lua_meta
    }

    // create Lua "meta" global variable
    // pub(super) fn lua_set_var(&self, lua: &MLua, globals: &Table<'_>) -> mlua::Result<()> {
    //     Lua::lua_set_var(lua, globals, self, "meta")
    // }

    // return the lua script from the luaproc attribute
    pub(super) fn lua_script(&self) -> Option<String> {
        let attr = self
            .attributes
            .iter()
            .find(|attr| attr.name == Some("luaproc".to_string()));

        attr.map(|attr| attr.inner.clone())
    }
}
