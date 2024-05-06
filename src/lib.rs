use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
use table::TableInfo;

mod field;
mod permission;
mod table;
mod type_conv;

#[proc_macro_derive(
    SurQLDefinition,
    attributes(
        surql_query,
        surql_table,
        surql_table_permissions,
        surql_field,
        surql_field_permissions
    )
)]
pub fn surql_definition_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let table_info = TableInfo::from_derive_input(&input);
    let struct_name = &input.ident;
    let method_name = format_ident!("schema_query");
    let query = table_info.generate_define_query();

    let expanded = quote! {
        impl #struct_name {
            pub fn #method_name() -> String {
                let sql = #query;

                #[cfg(feature = "with_query_validation")]
                surrealdb::sql::parse(sql).unwrap();

                sql.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}
