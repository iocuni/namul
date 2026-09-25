use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let source_dir = Path::new("src/syntax/function");
    println!("cargo:rerun-if-changed={}", source_dir.display());

    let mut modules = Vec::new();
    for entry in fs::read_dir(source_dir).expect("failed to read builtin function directory") {
        let path = entry.expect("failed to read builtin function entry").path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let name = path.file_stem().and_then(|stem| stem.to_str()).unwrap();
        if name == "mod" || name.starts_with('_') {
            continue;
        }
        if !is_rust_identifier(name) {
            panic!("builtin module name `{name}` is not a Rust identifier");
        }
        println!("cargo:rerun-if-changed={}", path.display());
        modules.push((name.to_owned(), path));
    }
    modules.sort_by(|left, right| left.0.cmp(&right.0));

    let declarations = modules
        .iter()
        .map(|(name, path)| {
            let path = fs::canonicalize(path)
                .expect("failed to canonicalize builtin module path")
                .to_string_lossy()
                .replace('\\', "/");
            format!(r##"#[path = "{path}"] pub mod {name};"##)
        })
        .collect::<Vec<_>>();
    let entries = modules
        .iter()
        .map(|(name, _)| format!("    &{name}::BUILTIN,"))
        .collect::<Vec<_>>();
    let output = format!(
        "{}\n\npub const BUILTINS: &[&Builtin] = &[\n{}\n];\n",
        declarations.join("\n"),
        entries.join("\n")
    );
    let output_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("builtin_modules.rs");
    fs::write(output_path, output).expect("failed to write builtin module list");
}

fn is_rust_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('_' | 'a'..='z' | 'A'..='Z'))
        && chars.all(|character| matches!(character, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9'))
}
