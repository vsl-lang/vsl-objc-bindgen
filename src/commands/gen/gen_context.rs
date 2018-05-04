use std::io::{Write, BufWriter};
use std::fs::File;
use std::collections::HashSet;
use super::symbol_status::SymbolStatus;
use std::path::PathBuf;
use clang::*;

pub struct GenContext {
    symbols: HashSet<SymbolStatus>,
    vsl_out: BufWriter<File>,
    vtable_out: BufWriter<File>,
    itf_out: BufWriter<File>,
    impl_out: BufWriter<File>,
    abi_out: BufWriter<File>,
}

fn rename_ext(path: &String, ext: &str) -> PathBuf {
    let path = PathBuf::from(path);
    path.with_extension(ext)
}

impl GenContext {
    pub fn new(symbols: HashSet<SymbolStatus>, vsl_out_path: String, clang_out_path: String, includes: Vec<String>) -> GenContext {
        let vtable_path = rename_ext(&clang_out_path, "vtable.h");
        let itf_path = rename_ext(&clang_out_path, "itf.h");

        let mut ctx = GenContext {
            symbols: symbols,
            vsl_out: create_file!(vsl_out_path),
            vtable_out: create_file!(vtable_path.clone()),
            itf_out: create_file!(itf_path.clone()),
            impl_out: create_file!(rename_ext(&clang_out_path, "impl.m")),
            abi_out: create_file!(rename_ext(&clang_out_path, "abi.m"))
        };

        let vtable_name = unwrap_or_exit!(vtable_path.file_name().and_then(|s| s.to_str()), "Could not unwrap vtable path");
        let itf_name = unwrap_or_exit!(itf_path.file_name().and_then(|s| s.to_str()), "Could not unwrap itf path");

        ctx.itf_out.write_fmt(format_args!("#include \"{}\"\n\n", &vtable_name));
        ctx.impl_out.write_fmt(format_args!("#include \"{}\"\n\n", &itf_name));
        ctx.abi_out.write_fmt(format_args!("#include \"{}\"\n\n", &itf_name));

        for include in includes {
            ctx.vtable_out.write_fmt(format_args!("#import {}\n\n", include));
            ctx.itf_out.write_fmt(format_args!("#import {}\n\n", include));
        }

        ctx.abi_out.write_all(b"extern \"C\" {\n\n");

        ctx
    }

    pub fn finish(&mut self) {
        self.abi_out.write_all(b"}\n");
    }

    pub fn get_vsl(&mut self) -> &mut BufWriter<File> {
        &mut self.vsl_out
    }

    pub fn get_vtable(&mut self) -> &mut BufWriter<File> {
        &mut self.vtable_out
    }

    pub fn get_itf(&mut self) -> &mut BufWriter<File> {
        &mut self.itf_out
    }

    pub fn get_impl(&mut self) -> &mut BufWriter<File> {
        &mut self.impl_out
    }

    pub fn get_abi(&mut self) -> &mut BufWriter<File> {
        &mut self.abi_out
    }

    pub fn get_symbols(&mut self) -> &mut HashSet<SymbolStatus> {
        &mut self.symbols
    }

    pub fn get_class_name(source: &String) -> String {
        format!("__vsl_ocpp_{}", source)
    }

    pub fn get_vtable_name(source: &String) -> String {
        format!("__vsl_ocpp_{}_vtable", source)
    }

    pub fn mangle_entity(entity: &Entity) -> String {
        let source = unwrap_or_exit!(
            entity.get_name(),
            "Could not obtain name of entity"
        );

        let mangle_str = source
            .into_bytes()
            .iter()
            .map(|byte| format!("{:X}", byte))
            .collect::<Vec<String>>()
            .join("");

        format!("__vsl_ocpp_{}", mangle_str)
    }
}

macro_rules! ty_to_cstr {
    ($ty:expr, $ctx:expr) => {
        {
            let ty = $ty;
            let mut ty_str = match ty.map_or(TypeKind::Void, |t| t.get_kind()) {
                TypeKind::Void => "void".to_string(),
                _ => {
                    unwrap_or_exit!(ty.map(|t| t.get_display_name()), "Could not get return type for {}", $ctx.get_display_name().unwrap_or("nil".to_string()))
                }
            };

            ty_str
        }
    };
}

macro_rules! gen_fmt {
    ($root:ident, $func:ident, $($arg:tt)*) => {
        clamp_error!(
            GenContext::$func($root).write_fmt(format_args!($($arg)*)),
            "Write failed"
        )
    };
}

macro_rules! gen_all {
    ($root:ident, $func:ident, $arg:expr) => {
        clamp_error!(
            GenContext::$func($root).write_all($arg),
            "Write failed"
        )
    };
}
