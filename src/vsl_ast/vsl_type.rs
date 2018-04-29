use vsl_ast::VSLEntity;
use std::fmt;

pub enum VSLType {
    Primitive { name: String },
    Generic { name: String, types: Vec<VSLType> }
}

impl VSLEntity for VSLType {}
impl fmt::Display for VSLType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VSLType::Primitive { ref name } => write!(f, "{}", name),
            VSLType::Generic { ref name, ref types } => {
                let generic_params = types
                    .iter()
                    .map(|generic_type| generic_type.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "{}<{}>", name, generic_params)
            }
        }
    }
}
