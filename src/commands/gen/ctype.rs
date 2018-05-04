use clang::*;

pub struct CType<'a> {
    clang_type: Type<'a>
}

impl<'a> CType<'a> {

    pub fn new(clang_type: Type<'a>) -> CType<'a> {
        CType {
            clang_type: clang_type
        }
    }

    pub fn to_string_named(&self, name: &str) -> String {
        let mut ty_str = match self.clang_type.get_kind() {
            TypeKind::Void => "void".to_string(),
            _ => self.clang_type.get_display_name()
        };

        ty_str
    }

}

impl<'a> ToString for CType<'a> {
    fn to_string(&self) -> String {
        self.to_string_named("")
    }
}
