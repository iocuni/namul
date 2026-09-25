use super::{Builtin, BuiltinType};

pub const BUILTIN: Builtin = Builtin {
    name: "min",
    args: &[BuiltinType::I32, BuiltinType::I32],
    result: BuiltinType::I32,
    c_definition: r#"
int32_t min(int32_t a,int32_t b) {
    if (a > b) return b;
    return a;
}
"#,
};
