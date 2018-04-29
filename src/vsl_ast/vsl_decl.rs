use vsl_ast::VSLEntity;
use std::hash::Hash;

pub trait VSLDecl: VSLEntity {
    fn get_name(&self) -> &String;
}
