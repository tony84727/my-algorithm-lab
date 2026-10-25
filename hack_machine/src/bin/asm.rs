use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use hack_machine::{codegen::generate, hack_file::encode_image, parser::parse};

#[derive(Parser)]
struct AsmArgs {
    program: PathBuf,
    #[arg(long)]
    ast: bool,
}

fn main() {
    let args = AsmArgs::parse();
    let source = read_to_string(args.program).expect("read source code");
    let (_, program) = parse(&source).expect("parse source code");
    if args.ast {
        println!("{:?}", program);
        return;
    }
    let image = generate(program).expect("generate code");
    println!("{}", encode_image(&image));
}
