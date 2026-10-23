use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use hack_machine::parser::parse;

#[derive(Parser)]
struct AsmArgs {
    program: PathBuf,
}

fn main() {
    let args = AsmArgs::parse();
    let source = read_to_string(args.program).expect("read source code");
    println!("{:?}", parse(&source));
}
