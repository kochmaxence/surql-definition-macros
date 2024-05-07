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
pub fn surreal_db_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match TableInfo::from_derive_input(&input) {
        Ok(table_info) => {
            let struct_name = &input.ident;
            let method_name = format_ident!("schema_query");
            let result = table_info.generate_define_query();
            let query = result.trim();

            #[cfg(feature = "compile_query_validation")]
            if let Err(e) = surrealdb_core::sql::parse(&query)
                .map_err(|err| syn::Error::new_spanned(&input, format!("{}\nQuery: {}", err, query)))
                .map_err(|err| TokenStream::from(err.into_compile_error()))
            {
                return e;
            };

            let expanded = quote! {
                impl SurQLSchemaProducer for #struct_name {
                    fn #method_name() -> &'static str {
                        const SQL: &'static str = concat!(#query);

                        #[cfg(feature = "runtime_query_validation")]
                        if let Err(e) = surrealdb_core::sql::parse(&SQL) {
                            panic!("{}", e.to_string());
                        }

                       SQL
                    }
                }
            };

            TokenStream::from(expanded)
        }
        Err(err) => TokenStream::from(err.into_compile_error()),
    }
}
