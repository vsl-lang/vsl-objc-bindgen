use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use vsl_ast::{VSLEntity, VSLDecl};
use std::fmt;

pub struct VSLAst {
    decls: Vec<Box<VSLDecl>>
}

impl VSLEntity for VSLAst {}
impl fmt::Display for VSLAst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let statements = self.decls
            .iter()
            .map(|statement| (**statement).to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", statements)
    }
}
