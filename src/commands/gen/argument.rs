use super::CType;

pub struct Argument<'a> {
    arg_ty: CType<'a>,
    private_name: Option<String>,
    public_name: String
}

impl<'a> Argument<'a> {
    pub fn new(public_name: String, arg_ty: CType<'a>) -> Argument<'a> {
        Argument {
            public_name: public_name,
            arg_ty: arg_ty,
            private_name: None
        }
    }

    pub fn get_type(&self) -> &CType<'a> {
        &self.arg_ty
    }

    pub fn get_public_name(&self) -> &String {
        &self.public_name
    }

    pub fn set_private_name(&mut self, private_name: String) {
        self.private_name = Some(private_name)
    }

    pub fn get_private_name(&self) -> &String {
        match self.private_name {
            Some(ref value) => value,
            None => &self.public_name
        }
    }

}
