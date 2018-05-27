use clang::*;
use std::io::Write;

use super::gen_context::GenContext;
use super::symbol_status::SymbolStatus;
use super::compile_entity_children;
use super::CType;

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
        let parent_class = unwrap_or_exit!(entity.get_semantic_parent(), "Could not locate pos {}", entity_name);
        let parent_ty = unwrap_or_exit!(parent_class.get_type().map(|t| t.get_display_name()), "Could not locate parent ty {}", entity_name);

        // Handle initializers seperately
        if entity_name.starts_with("init") {
            // We are an initializer
            let initializer_name = entity_name.replacen("init", "", 1);

            return
        }

        // Also split it into the components
        let mut method_args_iter = entity_name.split(":");
        let fn_name = unwrap_or_exit!(method_args_iter.next(), "Could not find function name for {}", entity_name);

        // Get return type
        let ret_ty = entity.get_result_type().map(|t| t.get_canonical_type());
        let ret_ty_cstr = ty_to_cstr!(ret_ty, entity);

        // Get arguments
        let fn_args = unwrap_or_exit!(entity.get_arguments(), "Could not get function args {}", entity_name);

        // Get public & private arg names
        let fn_public_arg_names = method_args_iter.collect::<Vec<&str>>();
        let fn_private_arg_names = fn_args
            .iter()
            .map(|arg|
                unwrap_or_exit!(
                    arg.get_name(),
                    "Could not get private fn arg name for {}", entity_name))
            .collect::<Vec<String>>();

        // Get argument types
        let fn_arg_types = fn_args.iter().map(|arg| arg.get_type()).collect::<Vec<Option<Type>>>();
        let fn_arg_cstrs = fn_arg_types
            .iter()
            .map(|arg_ty| ty_to_cstr!(arg_ty, entity))
            .collect::<Vec<String>>()
            .join(", ");

        // This this context, instance type is likely
        gen_fmt!(gen_context, get_vtable, "    {}(*{})({});\n", ret_ty_cstr, fn_name, fn_arg_cstrs);

        // Generate the fn proto
        let mut pub_names = fn_public_arg_names.clone();
        pub_names.insert(0, fn_name);

        let proto = if fn_private_arg_names.len() == 0 {
            fn_name.clone().to_string()
        } else {
            fn_private_arg_names
                .iter()
                .zip(pub_names.iter())
                .zip(fn_arg_types.iter())
                .map(|((priv_name, pub_name), ty)| format!("{}:({}){}", pub_name, ty_to_cstr!(ty, entity), priv_name))
                .collect::<Vec<String>>()
                .join(" ")
        };

        // Add the vtable call
        gen_fmt!(gen_context, get_impl, "- ({}){} {{\n    ", ret_ty_cstr, proto);

        // The local argument names
        let arg_names = fn_private_arg_names.join(", ");

        // Generates the call
        let super_call = if fn_private_arg_names.len() == 0 {
            fn_name.clone().to_string()
        } else {
            fn_private_arg_names
                .iter()
                .enumerate()
                .map(|(index, priv_arg)| format!("{}: {}", pub_names[index], priv_arg))
                .collect::<Vec<String>>()
                .join(" ")
        };

        gen_fmt!(gen_context, get_impl, "[super {}];\n    ", super_call);

        // Determine if to return.
        let does_return = ret_ty.map(|t| t.get_kind()).unwrap_or(TypeKind::Void) != TypeKind::Void;
        if does_return {
            gen_all!(gen_context, get_impl, b"return ");
        }

        gen_fmt!(gen_context, get_impl, "vtable->{}({});\n", fn_name, arg_names);

        gen_all!(gen_context, get_impl, b"}\n\n");

        // Write the ABI
        let mangled_name = GenContext::mangle_entity(&entity);

        gen_fmt!(gen_context, get_abi, "// {}\n", entity_name);
        if fn_private_arg_names.len() == 0 {
            gen_fmt!(gen_context, get_abi, "inline {} {}({}* __vsl_ocpp_self) {{\n    ", ret_ty_cstr, mangled_name, parent_ty);
        } else {
            let mangled_args = fn_private_arg_names
                .iter()
                .zip(fn_arg_types.iter())
                .map(|(name, ty)| {
                    let ty_string = ty_to_cstr!(ty, entity);
                    if ty_string == "void (^)()" {
                        format!("void (^{})()", name)
                    } else {
                        format!("{} {}", ty_string, name)
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");
            gen_fmt!(gen_context, get_abi, "inline {} {}({}* __vsl_ocpp_self, {}) {{\n    ", ret_ty_cstr, mangled_name, parent_ty, mangled_args);
        }

        if does_return {
            gen_all!(gen_context, get_abi, b"return ")
        }

        // If no args then we won't use formatting
        if fn_private_arg_names.len() == 0 {
            gen_fmt!(gen_context, get_abi, "[__vsl_ocpp_self {} /* (void) */];\n", fn_name);
        } else {
            let pub_name_mappings = pub_names
                .iter()
                .zip(fn_private_arg_names)
                .map(|(pub_name, priv_name)| format!("{}:{}", pub_name, priv_name))
                .collect::<Vec<String>>()
                .join(" ");
            gen_fmt!(gen_context, get_abi, "[__vsl_ocpp_self {}];\n", pub_name_mappings);
        }

        gen_all!(gen_context, get_abi, b"}\n\n");
    } else {
        warn!("Could not get name for target resolution entity.");
    }
}
