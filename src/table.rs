use syn::DeriveInput;

use crate::{
    field::{parse_fields, FieldInfo},
    permission::{format_permissions, parse_permissions_attributes, PermissionInfo},
};

pub(crate) struct TableInfo {
    custom_query: Option<String>,
    table_name: String,
    permissions: Option<String>,
    fields: Vec<FieldInfo>,
}

impl TableInfo {
    pub(crate) fn from_derive_input(input: &DeriveInput) -> Self {
        let (custom_query, table_name, permissions) = parse_table_attributes(input);
        let fields = parse_fields(input);

        TableInfo {
            custom_query,
            table_name,
            permissions,
            fields,
        }
    }

    pub(crate) fn generate_define_query(&self) -> String {
        self.custom_query.clone().unwrap_or_else(|| {
            let fields_def = self
                .fields
                .iter()
                .map(|field| field.generate_define_query(&self.table_name))
                .collect::<Vec<_>>()
                .join(" ");

            let mut query = format!("DEFINE TABLE {};", self.table_name);
            if let Some(perms) = &self.permissions {
                query = format!("DEFINE TABLE {} PERMISSIONS {};", self.table_name, perms);
            }
            format!("{} {}", query, fields_def)
        })
    }
}

fn parse_table_attributes(input: &DeriveInput) -> (Option<String>, String, Option<String>) {
    let mut custom_query = None;
    let mut explicit_table_name = None;

    let mut perms: Vec<PermissionInfo> = vec![];

    for attr in &input.attrs {
        if attr.path().is_ident("surql_query") {
            let lit: syn::LitStr = attr.parse_args().expect("Expected a string literal");
            custom_query = Some(lit.value());
        } else if attr.path().is_ident("surql_table") {
            let lit: syn::LitStr = attr.parse_args().expect("Expected a string literal");
            explicit_table_name = Some(lit.value());
        } else if attr.path().is_ident("surql_table_permissions") {
            attr.parse_nested_meta(|meta| {
                perms.push(parse_permissions_attributes(meta).unwrap());
                Ok(())
            })
            .expect("Failed to parse table permissions attribute");
        }
    }

    let table_name = explicit_table_name.unwrap_or_else(|| input.ident.to_string().to_lowercase());
    let permissions = format_permissions(perms);

    (custom_query, table_name, permissions)
}
