use super::CType;

pub struct Argument<'a> {
    arg_ty: CType<'a>,
    private_name: Option<&'a str>,
    public_name: &'a str
}

impl<'a> Argument<'a> {
    fn new(public_name: &'a str, arg_ty: CType<'a>) -> Argument<'a> {
        Argument {
            public_name: public_name,
            arg_ty: arg_ty,
            private_name: None
        }
    }
}
