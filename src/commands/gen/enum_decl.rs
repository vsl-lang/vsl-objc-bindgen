use clang::*;
use std::io::Write;

#[macro_use] use super::gen_context::*;
use super::symbol_status::SymbolStatus;
use super::compile_entity_children;

pub fn gen(gen_context: &mut GenContext, entity: Entity) {
    if let Some(entity_name) = entity.get_name() {

        if let Some(enum_ty) = entity.get_enum_underlying_type() {

        } else {
            warn!("Could not obtain enum type");
        }

    } else {
        warn!("Could not get name for target resolution entity.");
    }
}
