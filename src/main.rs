use translate::codegen::{Codegen, CodegenOptions};
use winnow::{Located, Parser};

mod args;
mod syntax;
mod translate;

fn main() {
    let args = {
        use clap::Parser;
        args::Args::parse()
    };
    let source = args.source.expect("a source file is required unless --lsp is used");
    let source_content = std::fs::read_to_string(source).unwrap();
    let input = Located::new(source_content.as_str());
    let output = syntax::parse_program.parse(input).unwrap();
    let c = Codegen::translate(
        &output,
        CodegenOptions {
            no_libc: args.no_libc,
        },
    );
    print!("{c}");
}
