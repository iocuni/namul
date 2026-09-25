use super::{Builtin, BuiltinType};

pub const BUILTIN: Builtin = Builtin {
    name: "asdf",
    args: &[],
    result: BuiltinType::I32,
    c_definition: r#"
int32_t asdf(void) {
    return 0;
}
"#,
};
