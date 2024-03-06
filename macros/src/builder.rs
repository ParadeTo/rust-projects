use proc_macro2::Ident;

use syn::{Field, Type};

struct Fd {
    name: Ident,
    ty: Type,
    optional: bool,
}

pub struct BuilderContext {
    name: Ident,
    fields: Vec<Fd>,
}

impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(f.ty);
        Self {
            name: f.ident.unwrap(),
            optional,
            ty,
        }
    }
}

fn get_option_inner(ty: Type) -> (bool, Type) {
    todo!()
}
