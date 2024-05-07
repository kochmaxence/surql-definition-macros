use syn::{meta::ParseNestedMeta, Data, DeriveInput, Field, Lit};

use crate::{
    permission::{parse_permissions_attributes, PermissionInfo},
    type_conv::{format_lit_as_expr, SurrealDBType},
};

pub(crate) struct FieldInfo {
    name: String,
    field_type: SurrealDBType,
    type_is_flexible: bool,
    default: Option<String>,
    readonly: bool,
    value: Option<String>,
    assertion: Option<String>,
    permissions: Vec<PermissionInfo>,
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
            &self.permissions,
        )
    }
}

impl From<&Field> for FieldInfo {
    fn from(f: &Field) -> Self {
        let name = f.ident.as_ref().unwrap().to_string();
        let mut field_type = SurrealDBType::from(&f.ty);
        let mut type_is_flexible = false;
        let mut assertion = None;
        let mut default = None;
        let mut readonly = false;
        let mut value = None;
        let mut permissions: Vec<PermissionInfo> = vec![];

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
                    permissions.push(parse_permissions_attributes(meta).unwrap());
                    Ok(())
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
            permissions,
        }
    }
}

pub(crate) fn parse_fields(input: &DeriveInput) -> Vec<FieldInfo> {
    if let Data::Struct(data_struct) = &input.data {
        data_struct.fields.iter().map(FieldInfo::from).collect()
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
    permissions: &Vec<PermissionInfo>,
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

    let permissions_str = permissions
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    if !permissions.is_empty() {
        define_field.push_str(" PERMISSIONS ");
        define_field.push_str(&permissions_str.join(" "));
    }

    if !define_field.ends_with(";") {
        define_field.push_str(";");
    }

    define_field
}
