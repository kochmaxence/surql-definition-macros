use std::fmt;

use syn::{meta::ParseNestedMeta, Lit};

use crate::type_conv::format_lit_as_expr;

pub(crate) struct PermissionData {
    value: String,
}

impl From<String> for PermissionData {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl Into<String> for PermissionData {
    fn into(self) -> String {
        self.value
    }
}

impl ToString for PermissionData {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

pub(crate) enum PermissionInfo {
    Select(PermissionData),
    Create(PermissionData),
    Update(PermissionData),
    Delete(PermissionData),
}

impl TryFrom<ParseNestedMeta<'_>> for PermissionInfo {
    type Error = syn::Error;

    fn try_from(meta: ParseNestedMeta<'_>) -> Result<Self, Self::Error> {
        let attribute_name = meta
            .path
            .get_ident()
            .map(|ident| ident.to_string())
            .unwrap_or_default();

        let lit: Lit = meta.value()?.parse()?;
        let perm_value = format_lit_as_expr(lit);

        match attribute_name.as_str() {
            "SELECT" => Ok(Self::Select(PermissionData::from(perm_value))),
            "CREATE" => Ok(Self::Create(PermissionData::from(perm_value))),
            "UPDATE" => Ok(Self::Update(PermissionData::from(perm_value))),
            "DELETE" => Ok(Self::Delete(PermissionData::from(perm_value))),
            _ => Err(meta.error("Unrecognized permission type")),
        }
    }
}

impl fmt::Display for PermissionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_string: String = match self {
            PermissionInfo::Select(inner) => format!("FOR select {}", inner.to_string()),
            PermissionInfo::Create(inner) => format!("FOR create {}", inner.to_string()),
            PermissionInfo::Update(inner) => format!("FOR update {}", inner.to_string()),
            PermissionInfo::Delete(inner) => format!("FOR delete {}", inner.to_string()),
        };
        write!(f, "{}", formatted_string)
    }
}

impl Into<String> for PermissionInfo {
    fn into(self) -> String {
        self.to_string()
    }
}

pub(crate) fn parse_permissions_attributes(meta: ParseNestedMeta) -> syn::Result<PermissionInfo> {
    PermissionInfo::try_from(meta)
}

pub(crate) fn format_permissions(permissions: Vec<PermissionInfo>) -> Option<String> {
    let permissions_str: Vec<String> = permissions
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>();

    if permissions_str.is_empty() {
        None
    } else {
        Some(permissions_str.join(" "))
    }
}
