use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{meta::ParseNestedMeta, parse_macro_input, Attribute, Data, DeriveInput, Lit, Type};

/// A procedural macro to derive SurrealDB table and field definitions
/// from Rust structs and their associated attributes.
///
/// # Attributes
/// - `surql_query`: Specifies a custom SurrealQL query.
/// - `surql_table`: Specifies the table name.
/// - `surql_table_permissions`: Specifies the table permissions.
/// - `surql_field`: Specifies field attributes.
/// - `surql_field_permissions`: Specifies field-level permissions.
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
    let struct_name = &input.ident;

    // Parse custom attributes for the table
    let (custom_query, table_name, permissions) = parse_table_attributes(&input);

    // Generate the query or use the provided one
    let query = custom_query
        .unwrap_or_else(|| generate_define_query(&input.data, &table_name, permissions.as_deref()));

    let method_name = format_ident!("schema_query");
    let expanded = quote! {
        impl #struct_name {
            /// Returns the SurrealQL query to define the table and its fields.
            pub fn #method_name() -> String {
                let sql = #query;
                sql.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Parses the table attributes from the derive input.
///
/// Returns a tuple containing the custom query, table name, and permissions.
fn parse_table_attributes(input: &DeriveInput) -> (Option<String>, String, Option<String>) {
    let mut custom_query = None;
    let mut explicit_table_name = None;
    let mut select_perm = None;
    let mut create_perm = None;
    let mut update_perm = None;
    let mut delete_perm = None;

    for attr in &input.attrs {
        if attr.path().is_ident("surql_query") {
            let lit: syn::LitStr = attr.parse_args().expect("Expected a string literal");
            custom_query = Some(lit.value());
        } else if attr.path().is_ident("surql_table") {
            let lit: syn::LitStr = attr.parse_args().expect("Expected a string literal");
            explicit_table_name = Some(lit.value());
        } else if attr.path().is_ident("surql_table_permissions") {
            attr.parse_nested_meta(|meta| {
                parse_permissions_attributes(
                    meta,
                    &mut select_perm,
                    &mut create_perm,
                    &mut update_perm,
                    &mut delete_perm,
                )
            })
            .expect("Failed to parse table permissions attribute");
        }
    }

    let table_name = explicit_table_name.unwrap_or_else(|| input.ident.to_string().to_lowercase());
    let permissions = format_permissions(select_perm, create_perm, update_perm, delete_perm);

    (custom_query, table_name, permissions)
}

/// Generates a SurrealQL `DEFINE TABLE` query from the given data, table name, and permissions.
fn generate_define_query(data: &Data, table_name: &str, permissions: Option<&str>) -> String {
    match data {
        Data::Struct(data_struct) => {
            let fields = data_struct.fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap().to_string();
                let surrealdb_type = rust_type_to_surrealdb_type(&f.ty);
                generate_define_field(&field_name, table_name, surrealdb_type, &f.attrs)
            });

            let mut query = format!("DEFINE TABLE {};", table_name);
            if let Some(perms) = permissions {
                query = format!("DEFINE TABLE {} PERMISSIONS {};", table_name, perms);
            }
            format!("{} {}", query, fields.collect::<Vec<_>>().join(" "))
        }
        _ => panic!("Unsupported data type"),
    }
}

/// Converts a Rust type to a SurrealDB type.
fn rust_type_to_surrealdb_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let type_ident = type_path.path.segments.first().unwrap().ident.to_string();
            match type_ident.as_str() {
                "i8" | "i16" | "i32" | "u8" | "u16" | "u32" => "int".to_string(),
                "i64" | "u64" => "number".to_string(),
                "f32" | "f64" => "float".to_string(),
                "bool" => "bool".to_string(),
                "char" | "String" => "string".to_string(),
                "Vec" => {
                    let inner_type = type_path
                        .path
                        .segments
                        .last()
                        .unwrap()
                        .arguments
                        .to_token_stream()
                        .to_string();
                    format!("array<{}>", inner_type)
                }
                "Option" => {
                    let inner_type = type_path
                        .path
                        .segments
                        .last()
                        .unwrap()
                        .arguments
                        .to_token_stream()
                        .to_string();
                    format!("option<{}>", inner_type)
                }
                _ => "record".to_string(),
            }
        }
        _ => panic!("Unsupported type"),
    }
}

/// Generates a `DEFINE FIELD` query for a given field, table, and attributes.
fn generate_define_field(
    field_name: &str,
    table_name: &str,
    mut field_type: String,
    attrs: &[Attribute],
) -> String {
    let mut type_is_flexible = false;
    let mut assertion = None;
    let mut default = None;
    let mut readonly = false;
    let mut value = None;
    let mut select_perm = None;
    let mut create_perm = None;
    let mut update_perm = None;
    let mut delete_perm = None;

    // Parse the attributes for the field
    for attr in attrs {
        if attr.path().is_ident("surql_field") {
            attr.parse_nested_meta(|meta| {
                parse_field_attributes(
                    meta,
                    &mut type_is_flexible,
                    &mut field_type,
                    &mut assertion,
                    &mut default,
                    &mut readonly,
                    &mut value,
                )
            })
            .expect("Failed to parse field attribute");
        } else if attr.path().is_ident("surql_field_permissions") {
            attr.parse_nested_meta(|meta| {
                parse_permissions_attributes(
                    meta,
                    &mut select_perm,
                    &mut create_perm,
                    &mut update_perm,
                    &mut delete_perm,
                )
            })
            .expect("Failed to parse field permissions attribute");
        }
    }

    // Build the DEFINE FIELD query
    build_define_field_query(
        field_name,
        table_name,
        &field_type,
        type_is_flexible,
        &default,
        readonly,
        &value,
        &assertion,
        &select_perm,
        &create_perm,
        &update_perm,
        &delete_perm,
    )
}

/// Parses the attributes for a given field.
fn parse_field_attributes(
    meta: ParseNestedMeta,
    type_is_flexible: &mut bool,
    field_type: &mut String,
    assertion: &mut Option<String>,
    default: &mut Option<String>,
    readonly: &mut bool,
    value: &mut Option<String>,
) -> syn::Result<()> {
    let attribute_name = meta
        .path
        .get_ident()
        .map(|ident| ident.to_string())
        .unwrap_or_default();
    match attribute_name.as_str() {
        "FLEXIBLE" => {
            *type_is_flexible = true;
            Ok(())
        }
        "READONLY" => {
            *readonly = true;
            Ok(())
        }
        "TYPE" => {
            let lit: Lit = meta.value()?.parse()?;
            match lit {
                Lit::Str(lit_str) => {
                    *field_type = lit_str.value();
                    Ok(())
                }
                _ => Err(meta.error("Expected a string for TYPE")),
            }
        }
        "ASSERT" | "DEFAULT" | "VALUE" => {
            let lit: Lit = meta.value()?.parse()?;
            let formatted_expr = format_lit_as_expr(lit);

            match attribute_name.as_str() {
                "ASSERT" => *assertion = Some(formatted_expr),
                "DEFAULT" => *default = Some(formatted_expr),
                "VALUE" => *value = Some(formatted_expr),
                _ => unreachable!(),
            }

            Ok(())
        }
        _ => Err(meta.error("Unrecognized field attribute")),
    }
}

/// Formats a literal into a string representation of a SurrealDB expression.
fn format_lit_as_expr(lit: Lit) -> String {
    match lit {
        Lit::Str(lit_str) => {
            let val = lit_str.value();
            val
        }
        Lit::Bool(lit_bool) => lit_bool.value().to_string(),
        _ => lit.to_token_stream().to_string(),
    }
}

/// Parses the permissions attributes for a table or field.
fn parse_permissions_attributes(
    meta: ParseNestedMeta,
    select_perm: &mut Option<String>,
    create_perm: &mut Option<String>,
    update_perm: &mut Option<String>,
    delete_perm: &mut Option<String>,
) -> syn::Result<()> {
    let attribute_name = meta
        .path
        .get_ident()
        .map(|ident| ident.to_string())
        .unwrap_or_default();

    let lit: Lit = meta.value()?.parse()?;
    let perm_value = format_lit_as_expr(lit);

    match attribute_name.as_str() {
        "SELECT" => {
            *select_perm = Some(perm_value);
            Ok(())
        }
        "CREATE" => {
            *create_perm = Some(perm_value);
            Ok(())
        }
        "UPDATE" => {
            *update_perm = Some(perm_value);
            Ok(())
        }
        "DELETE" => {
            *delete_perm = Some(perm_value);
            Ok(())
        }
        _ => Err(meta.error("Unrecognized permission type")),
    }
}

/// Formats permissions for a table or field into a single string.
fn format_permissions(
    select_perm: Option<String>,
    create_perm: Option<String>,
    update_perm: Option<String>,
    delete_perm: Option<String>,
) -> Option<String> {
    let permissions = vec![
        select_perm.map(|perm| format!("FOR select {}", perm)),
        create_perm.map(|perm| format!("FOR create {}", perm)),
        update_perm.map(|perm| format!("FOR update {}", perm)),
        delete_perm.map(|perm| format!("FOR delete {}", perm)),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect::<Vec<_>>();

    if permissions.is_empty() {
        None
    } else {
        Some(permissions.join(" "))
    }
}

/// Builds a `DEFINE FIELD` query for a given field, table, and its attributes.
fn build_define_field_query(
    field_name: &str,
    table_name: &str,
    field_type: &str,
    type_is_flexible: bool,
    default: &Option<String>,
    readonly: bool,
    value: &Option<String>,
    assertion: &Option<String>,
    select_perm: &Option<String>,
    create_perm: &Option<String>,
    update_perm: &Option<String>,
    delete_perm: &Option<String>,
) -> String {
    let mut define_field = format!("DEFINE FIELD {} ON {}", field_name, table_name);

    if type_is_flexible {
        define_field.push_str(" FLEXIBLE");
    }
    if !field_type.is_empty() {
        define_field.push_str(&format!(" TYPE {}", field_type));
    }
    if let Some(default) = default {
        define_field.push_str(&format!(" DEFAULT {}", default));
    }
    if readonly {
        define_field.push_str(" READONLY");
    }
    if let Some(value) = value {
        define_field.push_str(&format!(" VALUE {}", value));
    }
    if let Some(assertion) = assertion {
        define_field.push_str(&format!(" ASSERT {}", assertion));
    }

    let permissions = vec![
        select_perm
            .as_ref()
            .map(|perm| format!("FOR select {}", perm)),
        create_perm
            .as_ref()
            .map(|perm| format!("FOR create {}", perm)),
        update_perm
            .as_ref()
            .map(|perm| format!("FOR update {}", perm)),
        delete_perm
            .as_ref()
            .map(|perm| format!("FOR delete {}", perm)),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect::<Vec<_>>();

    if !permissions.is_empty() {
        define_field.push_str(" PERMISSIONS ");
        define_field.push_str(&permissions.join(" "));
    }

    if !define_field.ends_with(";") {
        define_field.push_str(";");
    }

    define_field
}
