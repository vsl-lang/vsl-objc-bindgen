use clang::*;

pub struct CType<'a> {
    clang_type: Type<'a>
}

impl<'a> CType<'a> {

    pub fn new(clang_type: Type<'a>) -> CType<'a> {
        CType {
            clang_type: clang_type.get_canonical_type()
        }
    }

    pub fn get_underlying_type(&self) -> &Type<'a> {
        &self.clang_type
    }

    pub fn is_void(&self) -> bool {
        self.clang_type.get_kind() == TypeKind::Void
    }

    pub fn to_string_named(&self, name: &str) -> String {
        let mut ty_str = match self.clang_type.get_kind() {
            TypeKind::Void => "void".to_string(),
            _ => self.clang_type.get_display_name()
        };

        ty_str
    }

    pub fn to_vsl_string(&self) -> String {
        match self.clang_type.get_kind() {
            TypeKind::Void => "Void".to_string(),
            TypeKind::Bool => "Bool".to_string(),

            TypeKind::Nullptr => "Pointer<UInt8>".to_string(),

            TypeKind::ObjCObjectPointer |
            TypeKind::ObjCClass => self.clang_type.get_display_name(),

            TypeKind::CharS |
            TypeKind::Short |
            TypeKind::SChar |
            TypeKind::CharU |
            TypeKind::UChar |
            TypeKind::UShort |
            TypeKind::Int |
            TypeKind::UInt |
            TypeKind::Long |
            TypeKind::ULong |
            TypeKind::LongLong |
            TypeKind::ULongLong |
            TypeKind::Int128 |
            TypeKind::UInt128 |
            TypeKind::Half |
            TypeKind::Float16 |
            TypeKind::Float |
            TypeKind::Double |
            TypeKind::LongDouble |
            TypeKind::WChar => {
                format!(
                    "{}Int{}",
                    if self.clang_type.is_signed_integer() { "" } else { "U" },
                    clamp_error!(self.clang_type.get_sizeof(), "could not get sizeof {:?}", self.clang_type)
                )
            },

            _ => error_exit!("Unsupported vsl stringify type {:?}", self.clang_type)
        }
    }

}

impl<'a> ToString for CType<'a> {
    fn to_string(&self) -> String {
        self.to_string_named("")
    }
}

impl<'a> PartialEq for CType<'a> {
    fn eq(&self, other: &CType) -> bool {
        self.clang_type == other.clang_type
    }
}
