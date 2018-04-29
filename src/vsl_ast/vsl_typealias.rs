use vsl_ast::{VSLDecl, VSLEntity, VSLType};
use std::fmt;

pub struct VSLTypeAlias {
    name: String,
    referenced_type: VSLType,
}

impl VSLDecl for VSLTypeAlias {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl VSLEntity for VSLTypeAlias {

}

impl fmt::Display for VSLTypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "public typealias {} = {}", self.name, self.referenced_type)
    }
}
