use syn::{Error, Ident};

pub fn duplicate_field_error(field: &Ident) -> Error {
    Error::new(field.span(), format!("duplicate identifier '{}'", field))
}
