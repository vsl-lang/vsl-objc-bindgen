use clap::{ArgMatches};
use clang::*;
use std::collections::HashSet;
use artifacts::Artifact;

pub mod typedef;
pub mod interface_decl;

import!(symbol_status);
import!(gen_context);

pub fn cli(matches: &ArgMatches) {
    // Create Artifact
    let target_sys = matches.value_of("SYSTEM")
                            .map(|string: &str| string.to_owned())
                            .expect("No system received");

    let target_platform = matches.value_of("PLATFORM")
                                 .map(|string: &str| string.to_owned())
                                 .expect("No platform received");

    let platform_version = matches
        .value_of("platform-version")
        .map_or_else(|| Artifact::infer_version(&target_platform), |value: &str| value.to_string());

    let artifact = Artifact::new(target_sys, target_platform, platform_version, "x86_64".to_string());

    // Handle files
    let entry_header = unwrap_or_exit!(matches.value_of("ENTRY_HEADER"), "no entry header");
    let symbols = unwrap_or_exit!(matches.values_of_lossy("SYMBOLS"), "no symbols found.");
    let mut frameworks: Vec<String> = matches.values_of_lossy("frameworks").unwrap_or(vec![])
        .into_iter()
        .flat_map(|framework| vec!["-framework".to_string(), framework])
        .collect();

    let c_file_path = unwrap_or_exit!(matches.value_of("C_OUTPUT_FILE"), "No output C file");
    let vsl_file_path = unwrap_or_exit!(matches.value_of("VSL_OUTPUT_FILE"), "No output VSL file");

    let symbols: HashSet<SymbolStatus> = symbols.iter().map(|symbol_name| SymbolStatus::new(symbol_name.to_string())).collect();
    let mut gen_context = GenContext::new(symbols, vsl_file_path.to_string(), c_file_path.to_string());

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

    // Parse the entry header
    let mut clang_args = artifact.get_comp_args();
    clang_args.append(&mut frameworks);

    info!("Using arguments: {:?}", clang_args);
    info!("Parsing entry header {}", entry_header);
    let tu = match index.parser(entry_header)
        .arguments(&clang_args)
        .parse() {
        Ok(value) => value,
        Err(err) => error_exit!("parsing failed: {}", err)
    };

    tu.get_entity()
        .get_children()
        .iter()
        .for_each(|entity| {
            if entity.get_kind() == EntityKind::ObjCInterfaceDecl {
                println!("found decl {}", entity.get_name().unwrap_or("n/a".to_string()));
            }

            if let Some(entity_name) = entity.get_name() {
                // println!("{}", entity_name);
                let mut symbol_status = SymbolStatus::new(entity_name);
                if entity.is_declaration() {
                    // Check if the symbol should be compiled
                    if gen_context.get_symbols().get(&symbol_status).map_or(false, |s| s.should_compile()) {
                        gen_context.get_symbols().remove(&symbol_status);
                        symbol_status.set_compiled();
                        gen_context.get_symbols().insert(symbol_status);

                        compile_entity(&gen_context, entity.get_canonical_entity())
                    }
                }
            }
        });

    gen_context
        .get_symbols()
        .iter()
        .for_each(|symbol| {
            if symbol.should_compile() {
                warn!("Uncompiled symbol {}", &symbol.get_name())
            }
        })
}

fn compile_entity(gen_context: &GenContext, entity: Entity) {
    let entity_kind = entity.get_kind();
    match entity_kind {
        EntityKind::TypedefDecl => typedef::gen(&gen_context, entity),
        EntityKind::ObjCInterfaceDecl => interface_decl::gen(&gen_context, entity),
        _ => warn!("Unsupported entity type {:?} for symbol {}", entity_kind, entity.get_name().unwrap_or("nil".to_string()))
    }
}
