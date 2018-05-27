use clang::*;
use std::io::Write;

use super::gen_context::GenContext;
use super::symbol_status::SymbolStatus;
use super::compile_entity_children;
use super::CType;
use super::Argument;

pub fn gen(gen_context: &mut GenContext, entity: Entity) {
    if let Some(mut entity_name) = entity.get_name() {

        // Check if available
        match entity.get_availability() {
            Availability::Inaccessible |
            Availability::Unavailable => {
                return
            }
            _ => {}
        }

        // Check if a function or property
        if entity_name.ends_with(":") {
            // Remove the trailing `:`
            entity_name.pop();
        }

        // Get parent function name
        let parent_ty = CType::new(
            unwrap_or_exit!(entity.get_semantic_parent().and_then(|i| i.get_type()), "Could not locate pos {}", entity_name)
        );

        // Also split it into the components
        let mut method_args_iter = entity_name.split(":");

        let fn_name = unwrap_or_exit!(method_args_iter.next(), "Could not get function name for {}", entity_name);
        let mut fn_public_arg_names = method_args_iter.collect::<Vec<&str>>();
        fn_public_arg_names.insert(0, fn_name.clone());

        // Get arguments
        let fn_args = unwrap_or_exit!(entity.get_arguments(), "Could not get function args {}", entity_name);
        let fn_args = fn_args
            .iter()
            .enumerate()
            .map(|(index, arg)| {
                let mut argument = Argument::new(
                    fn_public_arg_names[index].to_string(),
                    CType::new(
                        unwrap_or_exit!(
                            arg.get_type(),
                            "Could not get argument type for {} at {}", entity_name, index
                        )
                    )
                );

                argument.set_private_name(
                    unwrap_or_exit!(arg.get_name(), "Couldn't get arg name for {} at {}", entity_name, index)
                );

                argument
            })
            .collect::<Vec<Argument>>();

        // Get name

        // Get return type
        let ret_ty = CType::new(
            unwrap_or_exit!(entity.get_result_type().map(|t| t.get_canonical_type()), "Could not get return type for {}", entity_name)
        );

        // The local argument names
        let fn_abi_vtable_args = fn_args
            .iter()
            .map(|arg| arg.get_private_name().to_string())
            .collect::<Vec<String>>()
            .join(", ");

        // Write the ABI
        let mangled_name = GenContext::mangle_entity(&entity);

        gen_fmt!(gen_context, get_abi, "// {}\n", entity_name);

        if fn_args.len() == 0 {
            gen_fmt!(gen_context, get_abi, "inline {} {}() {{\n    ", ret_ty.to_string(), mangled_name);
        } else {
            // Args for vtable entry
            let mangled_args = fn_args
                .iter()
                .map(|arg| {
                    let ty_string = arg.get_type().to_string();
                    let name = arg.get_private_name();

                    if ty_string == "void (^)()" {
                        format!("void (^{})()", name)
                    } else {
                        format!("{} {}", ty_string, name)
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");

            gen_fmt!(gen_context, get_abi, "inline {} {}({}) {{\n    ", ret_ty.to_string(), mangled_name, mangled_args);
        }

        let does_return = !ret_ty.is_void();
        if does_return {
            gen_all!(gen_context, get_abi, b"return ")
        }

        // If no args then we won't use formatting
        if fn_args.len() == 0 {
            gen_fmt!(gen_context, get_abi, "[{} {} /* (void) */];\n", parent_ty.to_string(), fn_name);
        } else {
            let fn_abi_call = fn_args
                .iter()
                .map(|arg| format!("{}:{}", arg.get_public_name(), arg.get_private_name()))
                .collect::<Vec<String>>()
                .join(" ");

            gen_fmt!(gen_context, get_abi, "[{} {}];\n", parent_ty.to_string(), fn_abi_call);
        }

        gen_all!(gen_context, get_abi, b"}\n\n");

        // Generate the VSL call
        let vsl_proto = fn_args
            .iter()
            .map(|arg| format!("{}: {}", arg.get_public_name(), arg.get_type().to_vsl_string()))
            .collect::<Vec<String>>()
            .join(", ");

        gen_fmt!(
            gen_context, get_vsl,
            "    static func {}({}) -> {} external({})\n",
            fn_name,
            vsl_proto,
            ret_ty.to_vsl_string(),
            mangled_name
        )

    } else {
        warn!("Could not get name for target resolution entity.");
    }
}
