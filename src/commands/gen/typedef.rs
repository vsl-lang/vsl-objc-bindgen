use clang::*;
use super::gen_context::GenContext;
use super::symbol_status::SymbolStatus;

pub fn gen(gen_context: &GenContext, entity: Entity) {
    if let Some(entity_name) = entity.get_name() {

        // let underlying_type = self.get_typedef_underlying_type();
        // gen_context.resolve_entity_type(underlying_type);

    } else {
        warn!("Could not get name for target resolution entity.");
    }
}
