use syn::{meta::ParseNestedMeta, Data, DeriveInput, Lit};

use crate::{
    permission::parse_permissions_attributes,
    type_conv::{format_lit_as_expr, rust_type_to_surrealdb_type, SurrealDBType},
};

pub(crate) struct FieldInfo {
    name: String,
    field_type: SurrealDBType,
    type_is_flexible: bool,
    default: Option<String>,
    readonly: bool,
    value: Option<String>,
    assertion: Option<String>,
    select_perm: Option<String>,
    create_perm: Option<String>,
    update_perm: Option<String>,
    delete_perm: Option<String>,
}

impl FieldInfo {
    pub(crate) fn generate_define_query(&self, table_name: &str) -> String {
        build_define_field_query(
            &self.name,
            table_name,
            &self.field_type,
            self.type_is_flexible,
            &self.default,
            self.readonly,
            &self.value,
            &self.assertion,
            &self.select_perm,
            &self.create_perm,
            &self.update_perm,
            &self.delete_perm,
        )
    }
}

pub(crate) fn parse_fields(input: &DeriveInput) -> Vec<FieldInfo> {
    if let Data::Struct(data_struct) = &input.data {
        data_struct
            .fields
            .iter()
            .map(|f| {
                let name = f.ident.as_ref().unwrap().to_string();
                let mut field_type = rust_type_to_surrealdb_type(&f.ty);
                let mut type_is_flexible = false;
                let mut assertion = None;
                let mut default = None;
                let mut readonly = false;
                let mut value = None;
                let mut select_perm = None;
                let mut create_perm = None;
                let mut update_perm = None;
                let mut delete_perm = None;

                for attr in &f.attrs {
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

                FieldInfo {
                    name,
                    field_type,
                    type_is_flexible,
                    default,
                    readonly,
                    value,
                    assertion,
                    select_perm,
                    create_perm,
                    update_perm,
                    delete_perm,
                }
            })
            .collect()
    } else {
        panic!("Unsupported data type")
    }
}

fn parse_field_attributes(
    meta: ParseNestedMeta,
    type_is_flexible: &mut bool,
    field_type: &mut SurrealDBType,
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
                    *field_type = SurrealDBType::from(lit_str.value());
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

fn build_define_field_query(
    field_name: &str,
    table_name: &str,
    field_type: &SurrealDBType,
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
    if !field_type.to_string().is_empty() {
        define_field.push_str(&format!(" TYPE {}", field_type.to_string()));
    }
    if let Some(default) = default {
        define_field.push_str(&format!(" DEFAULT {}", default));
    }
    if let Some(value) = value {
        define_field.push_str(&format!(" VALUE {}", value));
    }
    if readonly {
        define_field.push_str(" READONLY");
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
