use std::fs::File;
use std::collections::HashSet;
use super::symbol_status::SymbolStatus;

pub struct GenContext {
    symbols: HashSet<SymbolStatus>,
    vsl_out: File,
    c_out: File
}

impl GenContext {
    pub fn new(symbols: HashSet<SymbolStatus>, vsl_ast_path: String, clang_ast_path: String) -> GenContext {
        GenContext {
            symbols: symbols,
            vsl_out: create_file!(vsl_ast_path),
            c_out: create_file!(clang_ast_path),
        }
    }

    pub fn get_symbols(&mut self) -> &mut HashSet<SymbolStatus> {
        &mut self.symbols
    }
}
