use syn::{meta::ParseNestedMeta, Lit};

use crate::type_conv::format_lit_as_expr;

pub(crate) fn parse_permissions_attributes(
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

pub(crate) fn format_permissions(
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
