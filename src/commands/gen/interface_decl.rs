use clang::*;
use std::io::Write;

#[macro_use] use super::gen_context::*;
use super::symbol_status::SymbolStatus;
use super::compile_entity_children;

pub fn gen(gen_context: &mut GenContext, entity: Entity) {
    if let Some(entity_name) = entity.get_name() {

        // Write a Obj-C Interface binding
        gen_fmt!(gen_context, get_itf, "@interface {} : {} {{\n", GenContext::get_class_name(&entity_name), entity_name);
        gen_fmt!(gen_context, get_itf, "    {}* vtable;\n}}\n\n", GenContext::get_vtable_name(&entity_name));

        // Write a Obj-C Interface binding
        gen_all!(gen_context, get_vtable, b"typedef struct {\n");

        // Write the implementation binding
        gen_fmt!(gen_context, get_impl, "@implementation {}\n\n", GenContext::get_class_name(&entity_name));

        // Write the VSL binding
        gen_fmt!(gen_context, get_vsl, "public class {} {{\n\n", entity_name);

        let main_entity = match entity.get_kind() {
            EntityKind::ObjCProtocolDecl => entity,
            _ => unwrap_or_exit!(
                entity.get_type().and_then(|entity_type| entity_type.get_declaration()),
                "Failed to find decl for interface {}", entity_name
            )
        };

        compile_entity_children(gen_context, main_entity, true);

        // Close Obj-C interface
        gen_all!(gen_context, get_itf, b"@end\n\n");

        // Close Obj-C implementation
        gen_fmt!(gen_context, get_impl, "@end\n\n");

        // Close Obj-C vtable
        gen_fmt!(gen_context, get_vtable, "}} {};\n\n", GenContext::get_vtable_name(&entity_name));

        // Close VSL binding
        gen_all!(gen_context, get_vsl, b"}\n\n");

    } else {
        warn!("Could not get name for target resolution entity.");
    }
}
