use super::{Builtin, BuiltinType};

pub const BUILTIN: Builtin = Builtin {
    name: "max",
    args: &[BuiltinType::I32, BuiltinType::I32],
    result: BuiltinType::I32,
    c_definition: r#"
int32_t max(int32_t a,int32_t b) {
    if (a < b) return b;
    return a;
}
"#,
};
