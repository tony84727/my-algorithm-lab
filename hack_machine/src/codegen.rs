use std::collections::HashMap;

use crate::ast::{AArgs, CArgs, Expr, Jump, Operation, Program};

fn scan_label<'p>(p: &Program<'p>, table: &mut HashMap<&'p str, u16>) {
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
}

fn scan_variables<'p>(p: &Program<'p>, table: &mut HashMap<&'p str, u16>) {
    let mut offset = 16;
    for expr in p.0.iter() {
        match expr {
            Expr::AOp(AArgs::Symbol(name)) => {
                if table.contains_key(name) {
                    continue;
                }
                table.insert(name, offset);
                offset += 1;
            }
            _ => continue,
        };
    }
}

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
    scan_label(p, &mut table);
    scan_variables(p, &mut table);
    table
}

#[derive(Debug)]
pub enum GenerateErorr {
    SymbolNotFound(String),
}

const A_OP_MASK: u16 = 0x7FFF;
const CONTROL_BIT_OFFSET: u16 = 6;
const SET_A_BIT: u16 = 1_u16 << 12;

fn encode_c_op(c: CArgs) -> u16 {
    let base = 0xE000_u16;
    let compute = match c.operation {
        Operation::Zero => 0b101010 << CONTROL_BIT_OFFSET,
        Operation::One => 0b111111 << CONTROL_BIT_OFFSET,
        Operation::NegativeOne => 0b111010 << CONTROL_BIT_OFFSET,
        Operation::D => 0b001100 << CONTROL_BIT_OFFSET,
        Operation::A => 0b110000 << CONTROL_BIT_OFFSET,
        Operation::M => 0b110000 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::NotD => 0b001101 << CONTROL_BIT_OFFSET,
        Operation::NotA => 0b110000 << CONTROL_BIT_OFFSET,
        Operation::NotM => 0b110000 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::NegativeD => 0b001111 << CONTROL_BIT_OFFSET,
        Operation::NegativeA => 0b110011 << CONTROL_BIT_OFFSET,
        Operation::NegativeM => 0b110011 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::IncreaseD => 0b011111 << CONTROL_BIT_OFFSET,
        Operation::IncreaseA => 0b110111 << CONTROL_BIT_OFFSET,
        Operation::IncreaseM => 0b110111 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::DecreaseD => 0b001110 << CONTROL_BIT_OFFSET,
        Operation::DecreaseA => 0b110010 << CONTROL_BIT_OFFSET,
        Operation::DecreaseM => 0b110010 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::DPlusA => 0b000010 << CONTROL_BIT_OFFSET,
        Operation::DPlusM => 0b000010 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::DMinusA => 0b010011 << CONTROL_BIT_OFFSET,
        Operation::DMinusM => 0b010011 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::AMinusD => 0b000111 << CONTROL_BIT_OFFSET,
        Operation::MMinusD => 0b000111 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::DAndA => 0b000000 << CONTROL_BIT_OFFSET,
        Operation::DAndM => 0b000000 << CONTROL_BIT_OFFSET | SET_A_BIT,
        Operation::DOrA => 0b010101 << CONTROL_BIT_OFFSET,
        Operation::DOrM => 0b010101 << CONTROL_BIT_OFFSET | SET_A_BIT,
    };
    let jump = c
        .jump
        .map(|j| match j {
            Jump::Noop => 0,
            Jump::JGT => 0b001,
            Jump::JEQ => 0b010,
            Jump::JGE => 0b011,
            Jump::JLT => 0b100,
            Jump::JNE => 0b101,
            Jump::JLE => 0b110,
            Jump::JMP => 0b111,
        })
        .unwrap_or_default();
    base | compute | c.destination.map(|d| (d.0 as u16) << 3).unwrap_or_default() | jump
}

pub fn generate(p: Program) -> Result<Vec<u16>, GenerateErorr> {
    let program = p.strip_comments();
    let sym_table = build_symbol_table(&program);
    let mut result = vec![];
    for expr in program.0.into_iter() {
        match expr {
            Expr::AOp(AArgs::Address(address)) => result.push(A_OP_MASK & address),
            Expr::AOp(AArgs::Symbol(name)) => {
                let Some(address) = sym_table.get(name) else {
                    return Err(GenerateErorr::SymbolNotFound(String::from(name)));
                };
                result.push(A_OP_MASK & address);
            }
            Expr::COp(c) => result.push(encode_c_op(c)),
            _ => continue,
        };
    }
    Ok(result)
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
