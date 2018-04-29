#[derive(Eq, Hash)]
pub struct SymbolStatus {
    name: String,
    compiled: bool
}

impl SymbolStatus {
    pub fn new(name: String) -> SymbolStatus {
        SymbolStatus { name: name, compiled: false }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn should_compile(&self) -> bool {
        !self.compiled
    }

    pub fn set_compiled(&mut self) {
        self.compiled = true;
    }
}

impl PartialEq for SymbolStatus {
    fn eq(&self, other: &SymbolStatus) -> bool {
        self.name == other.name
    }
}
