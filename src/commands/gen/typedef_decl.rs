use clang::*;
use std::io::Write;

#[macro_use] use super::gen_context::*;
use super::symbol_status::SymbolStatus;
use super::compile_entity;

pub fn gen(gen_context: &mut GenContext, entity: Entity) {
    if let Some(entity_name) = entity.get_name() {
        let target_ty = unwrap_or_exit!(entity.get_typedef_underlying_type(), "Could not get type of typedef {}", entity_name)
            .get_canonical_type();


        // Check if `typedef enum` and therefore redirect to that
        let type_kind = target_ty.get_kind();
        match type_kind {
            TypeKind::Enum => {
                let decl = unwrap_or_exit!(target_ty.get_declaration(), "Could not compile enum w/o decl {}", entity_name);
                compile_entity(gen_context, decl);
            },

            _ => {
                warn!("Could not compile unknown typedef target ty {:?} of {}", type_kind, entity_name);
            }
        }

    } else {
        warn!("Could not get name for target resolution entity.");
    }
}
