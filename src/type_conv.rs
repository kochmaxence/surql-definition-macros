use quote::ToTokens as _;
use syn::{GenericArgument, Lit, PathArguments, Type};

pub(crate) fn format_lit_as_expr(lit: Lit) -> String {
    match lit {
        Lit::Str(lit_str) => {
            let val = lit_str.value();
            val
        }
        Lit::Bool(lit_bool) => lit_bool.value().to_string(),
        _ => lit.to_token_stream().to_string(),
    }
}

#[derive(Debug)]
pub(crate) enum SurrealDBType {
    Raw(String),
    Int,
    Number,
    Float,
    Bool,
    String,
    Array(Box<SurrealDBType>),
    Option(Box<SurrealDBType>),
    Record,
}

impl ToString for SurrealDBType {
    fn to_string(&self) -> String {
        match self {
            SurrealDBType::Int => "int".to_string(),
            SurrealDBType::Number => "number".to_string(),
            SurrealDBType::Float => "float".to_string(),
            SurrealDBType::Bool => "bool".to_string(),
            SurrealDBType::String => "string".to_string(),
            SurrealDBType::Array(inner) => format!("array<{}>", inner.to_string()),
            SurrealDBType::Option(inner) => format!("option<{}>", inner.to_string()),
            SurrealDBType::Record => "record".to_string(),
            SurrealDBType::Raw(inner) => inner.to_string(),
        }
    }
}

pub(crate) enum TypeFrom {
    Rust(String),
    SurrealDB(String),
}

impl From<TypeFrom> for SurrealDBType {
    fn from(value: TypeFrom) -> Self {
        match value {
            TypeFrom::Rust(inner) => todo!(),
            TypeFrom::SurrealDB(inner) => inner.into(),
        }
    }
}

impl From<String> for SurrealDBType {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "int" => SurrealDBType::Int,
            "number" => SurrealDBType::Number,
            "float" => SurrealDBType::Float,
            "bool" => SurrealDBType::Bool,
            "string" => SurrealDBType::String,
            "record" => SurrealDBType::Record,
            _ => SurrealDBType::Raw(value), // TODO: array<inner> option<inner>
        }
    }
}

impl From<&Type> for SurrealDBType {
    fn from(ty: &Type) -> Self {
        match ty {
            Type::Path(type_path) => {
                let path = &type_path.path;
                if let Some(segment) = path.segments.last() {
                    match segment.ident.to_string().as_str() {
                        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                        | "u64" | "u128" | "usize" => SurrealDBType::Int,
                        "f32" | "f64" => SurrealDBType::Float,
                        "bool" => SurrealDBType::Bool,
                        "String" => SurrealDBType::String,
                        "Option" => {
                            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(GenericArgument::Type(inner_ty)) =
                                    args.args.iter().next()
                                {
                                    return SurrealDBType::Option(Box::new(SurrealDBType::from(
                                        inner_ty,
                                    )));
                                }
                            }

                            panic!("Unsupported inner type")
                        }
                        "Vec" => {
                            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(GenericArgument::Type(inner_ty)) =
                                    args.args.iter().next()
                                {
                                    return SurrealDBType::Array(Box::new(SurrealDBType::from(
                                        inner_ty,
                                    )));
                                }
                            }

                            panic!("Unsupported inner type")
                        }
                        _ => panic!("Unsupported type"),
                    }
                } else {
                    panic!("Unsupported type")
                }
            }
            _ => panic!("Unsupported type"),
        }
    }
}

impl Into<String> for SurrealDBType {
    fn into(self) -> String {
        self.to_string()
    }
}
