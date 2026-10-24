use std::collections::HashMap;

use crate::ast::{Expr, Program};

fn build_symbol_table<'p>(p: &Program<'p>) -> HashMap<&'p str, u16> {
    let mut table = HashMap::new();
    table.insert("R0", 0);
    table.insert("R1", 1);
    table.insert("R2", 2);
    table.insert("R3", 3);
    table.insert("R4", 4);
    table.insert("R5", 5);
    table.insert("R6", 6);
    table.insert("R7", 7);
    table.insert("R8", 8);
    table.insert("R9", 9);
    table.insert("R10", 10);
    table.insert("R11", 11);
    table.insert("R12", 12);
    table.insert("R13", 13);
    table.insert("R14", 14);
    table.insert("R15", 15);
    table.insert("SP", 0);
    table.insert("LCL", 1);
    table.insert("ARG", 2);
    table.insert("THIS", 3);
    table.insert("THAT", 4);
    table.insert("SCREEN", 16384);
    table.insert("KBD", 24576);
    let mut i = 0;
    for exp in p.0.iter() {
        match exp {
            Expr::Label(name) => {
                if table.contains_key(*name) {
                    continue;
                }
                table.insert(*name, i);
            }
            _ => {
                i += 1;
            }
        }
    }
    table
}

pub fn generate(p: Program) -> Vec<u16> {
    let sym_table = build_symbol_table(&p);
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::{codegen::build_symbol_table, parser::parse};

    #[test]
    fn test_build_symbol_table_scan_labels() {
        let program_code = r#"
// this is a comment
A=A+1
(LOOP)
D=1
@R0
(END)
M=1
"#;
        let (_, program) = parse(&program_code).unwrap();
        let sym_table = build_symbol_table(&program.strip_comments());
        assert_eq!(1, *sym_table.get("LOOP").unwrap());
        assert_eq!(3, *sym_table.get("END").unwrap());
    }
}
