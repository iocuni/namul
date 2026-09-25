use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub source: Option<String>,
    #[clap(long)]
    pub no_libc: bool,
    #[clap(long)]
    pub lsp: bool,
}
