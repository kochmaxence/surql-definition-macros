use quote::ToTokens;
use std::fmt;
use syn::{Error, Result};
use syn::{GenericArgument, Lit, PathArguments, Type};

pub(crate) fn format_lit_as_expr(lit: Lit) -> String {
    match lit {
        Lit::Str(lit_str) => lit_str.value(),
        Lit::Bool(lit_bool) => lit_bool.value().to_string(),
        _ => lit.to_token_stream().to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SurrealDBType {
    pub name: String,
    pub inner: Option<Box<SurrealDBType>>,
}

impl SurrealDBType {
    pub fn new(name: &str, inner: Option<SurrealDBType>) -> Self {
        Self {
            name: name.to_string(),
            inner: inner.map(Box::new),
        }
    }

    pub fn from_string(value: &str) -> Self {
        match value {
            "int" => SurrealDBType::new("int", None),
            "number" => SurrealDBType::new("number", None),
            "float" => SurrealDBType::new("float", None),
            "bool" => SurrealDBType::new("bool", None),
            "string" => SurrealDBType::new("string", None),
            "record" => SurrealDBType::new("record", None),
            _ => {
                if value.starts_with("array<") && value.ends_with(">") {
                    let inner = &value[6..value.len() - 1];
                    SurrealDBType::new("array", Some(SurrealDBType::from_string(inner)))
                } else if value.starts_with("option<") && value.ends_with(">") {
                    let inner = &value[7..value.len() - 1];
                    SurrealDBType::new("option", Some(SurrealDBType::from_string(inner)))
                } else {
                    SurrealDBType::new(value, None)
                }
            }
        }
    }

    pub fn from_type(ty: &Type) -> Result<Self> {
        match ty {
            Type::Path(type_path) => {
                let path = &type_path.path;
                if let Some(segment) = path.segments.last() {
                    let name = segment.ident.to_string();
                    match name.as_str() {
                        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => Ok(SurrealDBType::new("int", None)),
                        "f32" | "f64" => Ok(SurrealDBType::new("float", None)),
                        "bool" => Ok(SurrealDBType::new("bool", None)),
                        "String" => Ok(SurrealDBType::new("string", None)),
                        "char" => Ok(SurrealDBType::new("string", None)),
                        "Option" => {
                            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(GenericArgument::Type(inner_ty)) = args.args.iter().next() {
                                    let inner = SurrealDBType::from_type(inner_ty)?;
                                    return Ok(SurrealDBType::new("option", Some(inner)));
                                }
                            }
                            Err(Error::new_spanned(segment, "Option type requires a single generic type argument"))
                        }
                        "Vec" => {
                            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(GenericArgument::Type(inner_ty)) = args.args.iter().next() {
                                    let inner = SurrealDBType::from_type(inner_ty)?;
                                    return Ok(SurrealDBType::new("array", Some(inner)));
                                }
                            }
                            Err(Error::new_spanned(segment, "Vec type requires a single generic type argument"))
                        }
                        _ => Err(Error::new_spanned(segment, format!("Unsupported type: {}. Consider defining this type explicitly using the TYPE statement.", segment.ident))),
                    }
                } else {
                    Err(Error::new_spanned(type_path, "Path segment is missing"))
                }
            }
            _ => Err(Error::new_spanned(ty, format!("Unsupported type: {}. Consider defining this type explicitly using the TYPE statement.", ty.to_token_stream()))),
        }
    }

    pub fn to_string(&self) -> String {
        if let Some(inner) = &self.inner {
            format!("{}<{}>", self.name, inner)
        } else {
            self.name.clone()
        }
    }
}

impl fmt::Display for SurrealDBType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
