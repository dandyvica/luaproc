use quote::ToTokens;
use serde::Serialize;
use syn::DataEnum;

use crate::{
    lua::{attributes, fields, Field, Variant},
    lua_meta::LuaMeta,
};

/// Defines the structures which are serialized and then passed to Lua script
/// as global tables.
#[derive(Debug, Default, Serialize)]
pub(super) struct LuaEnum {
    // List of structure fields
    variants: Vec<Variant>,

    // list of fields
    fields: Vec<Field>,

    pub(super) meta: LuaMeta,
}

impl LuaEnum {
    #[allow(clippy::field_reassign_with_default)]
    pub(super) fn new(ds: &DataEnum) -> Self {
        // this will act as the interface between Rust & Lua
        let mut lua_enum = Self::default();

        // lookup each variant
        for v in &ds.variants {
            let mut var = Variant::default();

            // attributes
            var.attributes = attributes(&v.attrs);

            // ident
            var.name = v.ident.to_string();

            // fields
            match &v.fields {
                syn::Fields::Named(fnamed) => {
                    var.fields = fields(&fnamed.named);
                }
                syn::Fields::Unnamed(funnamed) => {
                    var.fields = fields(&funnamed.unnamed);
                }
                syn::Fields::Unit => {}
            }

            // discriminant if any
            if let Some(disc) = &v.discriminant {
                let d = disc.1.to_token_stream().to_string();
                var.discriminant = Some(d);
            }

            lua_enum.variants.push(var);
        }

        lua_enum
    }
}
