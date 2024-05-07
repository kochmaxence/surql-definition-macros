use crate::{
    permission::{parse_permissions_attributes, PermissionInfo},
    type_conv::{format_lit_as_expr, SurrealDBType},
};
use syn::{Data, DeriveInput, Error, Field, Lit, Result};

#[derive(Clone)]
pub(crate) struct FieldInfo {
    name: String,
    field_type: Option<SurrealDBType>,
    type_is_flexible: bool,
    default: Option<String>,
    readonly: bool,
    value: Option<String>,
    assertion: Option<String>,
    permissions: Vec<PermissionInfo>,
}

impl FieldInfo {
    pub(crate) fn new(name: String) -> Self {
        FieldInfo {
            name,
            field_type: None,
            type_is_flexible: false,
            default: None,
            readonly: false,
            value: None,
            assertion: None,
            permissions: vec![],
        }
    }

    pub(crate) fn generate_define_query(&self, table_name: &str) -> String {
        let mut define_field = format!("DEFINE FIELD {} ON {}", self.name, table_name);

        if self.type_is_flexible {
            define_field.push_str(" FLEXIBLE");
        }

        if let Some(field_ty) = &self.field_type {
            if !field_ty.to_string().is_empty() {
                define_field.push_str(&format!(" TYPE {}", field_ty.to_string()));
            }
        }

        if let Some(default) = &self.default {
            define_field.push_str(&format!(" DEFAULT {}", default));
        }
        if let Some(value) = &self.value {
            define_field.push_str(&format!(" VALUE {}", value));
        }

        if self.readonly {
            define_field.push_str(" READONLY");
        }
        if let Some(assertion) = &self.assertion {
            define_field.push_str(&format!(" ASSERT {}", assertion));
        }

        let permissions_str = self
            .permissions
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        if !self.permissions.is_empty() {
            define_field.push_str(" PERMISSIONS ");
            define_field.push_str(&permissions_str.join(" "));
        }


        if !define_field.ends_with(";") {
            define_field.push_str(";");
        }

        define_field
    }

    pub(crate) fn from_field(f: &Field) -> Result<Self> {
        let name = f
            .ident
            .as_ref()
            .ok_or_else(|| Error::new_spanned(f, "Expected field to have an identifier"))?
            .to_string();

        let mut field_info = FieldInfo::new(name);

        for attr in &f.attrs {
            if attr.path().is_ident("surql_field") {
                field_info = field_info.parse_field_attributes(attr)?;
            } else if attr.path().is_ident("surql_field_permissions") {
                attr.parse_nested_meta(|meta| {
                    let perm = parse_permissions_attributes(meta).map_err(|e| {
                        Error::new_spanned(
                            attr,
                            format!("Failed to parse permissions attribute: {}", e),
                        )
                    })?;
                    field_info.permissions.push(perm);
                    Ok(())
                })
                .map_err(|e| {
                    Error::new_spanned(
                        attr,
                        format!("Failed to parse field permissions attribute: {}", e),
                    )
                })?;
            }
        }

        // Infer the type if not explicitly set
        if field_info.field_type.is_none() {
            field_info.field_type = Some(SurrealDBType::from_type(&f.ty)?);
        }

        Ok(field_info)
    }

    fn parse_field_attributes(&self, attr: &syn::Attribute) -> Result<Self> {
        let mut field_info = self.clone();

        attr.parse_nested_meta(|meta| {
            let attribute_name = meta
                .path
                .get_ident()
                .map(|ident| ident.to_string())
                .unwrap_or_default();
            match attribute_name.as_str() {
                "FLEXIBLE" => {
                    field_info.type_is_flexible = true;
                    Ok(())
                }
                "READONLY" => {
                    field_info.readonly = true;
                    Ok(())
                }
                "TYPE" => {
                    let lit: Lit = meta.value()?.parse()?;
                    match lit {
                        Lit::Str(lit_str) => {
                            field_info.field_type =
                                Some(SurrealDBType::from_string(lit_str.value().as_ref()));
                            Ok(())
                        }
                        _ => Err(meta.error("Expected a string for TYPE")),
                    }
                }
                "ASSERT" | "DEFAULT" | "VALUE" => {
                    let lit: Lit = meta.value()?.parse()?;
                    let formatted_expr = format_lit_as_expr(lit);

                    match attribute_name.as_str() {
                        "ASSERT" => field_info.assertion = Some(formatted_expr),
                        "DEFAULT" => field_info.default = Some(formatted_expr),
                        "VALUE" => field_info.value = Some(formatted_expr),
                        _ => unreachable!(),
                    }

                    Ok(())
                }
                _ => Err(meta.error("Unrecognized field attribute")),
            }
        })?;

        Ok(field_info)
    }

    pub fn parse_fields(input: &DeriveInput) -> Result<Vec<Self>> {
        if let Data::Struct(data_struct) = &input.data {
            data_struct
                .fields
                .iter()
                .map(|f| FieldInfo::from_field(f))
                .collect::<Result<Vec<_>>>()
        } else {
            Err(Error::new_spanned(input, "Unsupported data type"))
        }
    }
}
