use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, digit1, line_ending, newline, not_line_ending, one_of, space0},
    combinator::{map, map_res, opt, recognize, value, verify},
    error::ParseError,
    multi::{fold_many1, many0, separated_list0},
    sequence::{delimited, pair, preceded},
};

use crate::ast::{AArgs, CArgs, Destination, Expr, Jump, Operation, Program};

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(|c: char| c.is_ascii_alphabetic() || "$_.:".contains(c)),
        take_while(|c: char| c.is_ascii_alphanumeric() || "$_.:".contains(c)),
    ))
    .parse(input)
}

fn label(input: &str) -> IResult<&str, Expr<'_>> {
    map(delimited(tag("("), identifier, tag(")")), Expr::Label).parse(input)
}

fn a_command_address(input: &str) -> IResult<&str, AArgs<'_>> {
    map_res(digit1, |x: &str| x.parse::<u16>().map(AArgs::Address)).parse(input)
}

fn a_command_symbol(input: &str) -> IResult<&str, AArgs<'_>> {
    map(identifier, AArgs::Symbol).parse(input)
}

fn a_command(input: &str) -> IResult<&str, Expr<'_>> {
    map(
        preceded(tag("@"), alt((a_command_address, a_command_symbol))),
        Expr::AOp,
    )
    .parse(input)
}

fn comment(input: &str) -> IResult<&str, Expr<'_>> {
    map(preceded(tag("//"), not_line_ending), |x: &str| {
        Expr::Comment(x.trim())
    })
    .parse(input)
}

fn blank_line(input: &str) -> IResult<&str, ()> {
    nom::combinator::value((), (space0, line_ending)).parse(input)
}

fn ws<'a, O, E, P>(parser: P) -> impl Parser<&'a str, Output = O, Error = E>
where
    P: Parser<&'a str, Output = O, Error = E>,
    E: ParseError<&'a str>,
{
    delimited(space0, parser, space0)
}

fn d_dest(input: &str) -> IResult<&str, Destination> {
    map(
        verify(
            fold_many1(
                one_of("AMD"),
                || (0_u8, 0_usize),
                |(flag, count), c| {
                    let new_flag = match c {
                        'A' => 1 << 2,
                        'M' => 1,
                        'D' => 1 << 1,
                        _ => panic!("parser malfunction"),
                    };
                    (flag | new_flag, count + 1)
                },
            ),
            |(flag, count)| flag.count_ones() == (*count as u32),
        ),
        |(flag, _)| Destination(flag),
    )
    .parse(input)
}

fn constant_compute(input: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::Zero, ws(tag("0"))),
        value(Operation::One, ws(tag("1"))),
        value(Operation::NegativeOne, (ws(tag("-")), ws(tag("1")))),
        value(Operation::A, ws(tag("A"))),
        value(Operation::M, ws(tag("M"))),
        value(Operation::D, ws(tag("D"))),
    ))
    .parse(input)
}

fn increase_decrease_compute(input: &str) -> IResult<&str, Operation> {
    alt((
        value(
            Operation::IncreaseA,
            (ws(char('A')), ws(char('+')), ws(char('1'))),
        ),
        value(
            Operation::IncreaseM,
            (ws(char('M')), ws(char('+')), ws(char('1'))),
        ),
        value(
            Operation::IncreaseD,
            (ws(char('D')), ws(char('+')), ws(char('1'))),
        ),
        value(
            Operation::DecreaseA,
            (ws(char('A')), ws(char('-')), ws(char('1'))),
        ),
        value(
            Operation::DecreaseM,
            (ws(char('M')), ws(char('-')), ws(char('1'))),
        ),
        value(
            Operation::DecreaseD,
            (ws(char('D')), ws(char('-')), ws(char('1'))),
        ),
    ))
    .parse(input)
}

fn d_compute(input: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::NotA, (ws(char('!')), ws(char('A')))),
        value(Operation::NotM, (ws(char('!')), ws(char('M')))),
        value(Operation::NotD, (ws(char('!')), ws(char('D')))),
        value(Operation::NegativeA, (ws(char('-')), ws(char('A')))),
        value(Operation::NegativeM, (ws(char('-')), ws(char('M')))),
        value(Operation::NegativeD, (ws(char('-')), ws(char('D')))),
        value(
            Operation::DPlusA,
            (ws(char('D')), ws(char('+')), ws(char('A'))),
        ),
        value(
            Operation::DPlusM,
            (ws(char('D')), ws(char('+')), ws(char('M'))),
        ),
        value(
            Operation::DMinusA,
            (ws(char('D')), ws(char('-')), ws(char('A'))),
        ),
        value(
            Operation::AMinusD,
            (ws(char('A')), ws(char('-')), ws(char('D'))),
        ),
        value(
            Operation::DAndA,
            (ws(char('D')), ws(char('&')), ws(char('A'))),
        ),
        value(
            Operation::DOrA,
            (ws(char('D')), ws(char('|')), ws(char('A'))),
        ),
        value(
            Operation::DMinusM,
            (ws(char('D')), ws(char('-')), ws(char('M'))),
        ),
        value(
            Operation::MMinusD,
            (ws(char('M')), ws(char('-')), ws(char('D'))),
        ),
        value(
            Operation::DAndM,
            (ws(char('D')), ws(char('&')), ws(char('M'))),
        ),
        value(
            Operation::DOrM,
            (ws(char('D')), ws(char('|')), ws(char('M'))),
        ),
        increase_decrease_compute,
        constant_compute,
    ))
    .parse(input)
}

fn d_jump(input: &str) -> IResult<&str, Jump> {
    alt((
        value(Jump::JGT, tag("JGT")),
        value(Jump::JEQ, tag("JEQ")),
        value(Jump::JGE, tag("JGE")),
        value(Jump::JLT, tag("JLT")),
        value(Jump::JNE, tag("JNE")),
        value(Jump::JLE, tag("JLE")),
        value(Jump::JMP, tag("JMP")),
    ))
    .parse(input)
}

fn d_command(input: &str) -> IResult<&str, Expr<'_>> {
    map(
        (
            opt((d_dest, ws(tag("=")))),
            ws(d_compute),
            opt(preceded(ws(char(';')), ws(d_jump))),
        ),
        |(dest, comp, jump)| {
            Expr::COp(CArgs {
                destination: dest.map(|(x, _)| x),
                operation: comp,
                jump,
            })
        },
    )
    .parse(input)
}

fn expr(input: &str) -> IResult<&'_ str, Expr<'_>> {
    preceded(
        many0(blank_line),
        alt((comment, label, a_command, d_command)),
    )
    .parse(input)
}

pub fn parse(input: &str) -> IResult<&'_ str, Program<'_>> {
    map(separated_list0(newline, expr), Program).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{AArgs, CArgs, Destination, Expr, Jump, Operation, Program},
        parser::{d_command, d_compute, expr, parse},
    };

    #[test]
    fn test_label() {
        let (_remaining, parsed) = expr("(LOOP)").unwrap();
        assert_eq!(Expr::Label("LOOP"), parsed);
    }

    #[test]
    fn test_label_case_1() {
        let (_remaining, parsed) = expr("(END_LT)").unwrap();
        assert_eq!(Expr::Label("END_LT"), parsed);
    }

    #[test]
    fn test_a_command_address() {
        let (_remaining, parsed) = expr("@1234").unwrap();
        assert_eq!(Expr::AOp(AArgs::Address(1234)), parsed);
    }

    #[test]
    fn test_a_command_symbol() {
        let (_remaining, parsed) = expr("@screen").unwrap();
        assert_eq!(Expr::AOp(AArgs::Symbol("screen")), parsed);
    }

    #[test]
    fn test_comment() {
        let (_remaining, parsed) = expr("// this is a comment").unwrap();
        assert_eq!(Expr::Comment("this is a comment"), parsed,);
    }

    #[test]
    fn test_parse_compute() {
        let (_remaining, parsed) = d_compute("-1").unwrap();
        assert_eq!(Operation::NegativeOne, parsed);
    }

    #[test]
    fn test_parse_compute_negative_one_with_random_spaces() {
        let (_remaining, parsed) = d_compute("- 1").unwrap();
        assert_eq!(Operation::NegativeOne, parsed);
    }

    #[test]
    fn test_d_command_parsing() {
        let (_remaining, parsed) = d_command("MD = 1").unwrap();
        assert_eq!(
            Expr::COp(CArgs {
                destination: Some(Destination(3)),
                operation: Operation::One,
                jump: None
            }),
            parsed
        );
    }

    #[test]
    fn test_d_command_parsing_1() {
        let (_remaining, parsed) = d_command("AMD = D+1; JLE").unwrap();
        assert_eq!(
            Expr::COp(CArgs {
                destination: Some(Destination(7)),
                operation: Operation::IncreaseD,
                jump: Some(Jump::JLE),
            }),
            parsed
        );
    }

    #[test]
    fn test_parse_program_with_comment_and_empty_lines() {
        let (_remaining, parsed) = parse(
            r#"// this is a comment

@1
A=A+1
        "#,
        )
        .unwrap();
        assert_eq!(
            Program(vec![
                Expr::Comment("this is a comment"),
                Expr::AOp(AArgs::Address(1)),
                Expr::COp(CArgs {
                    destination: Some(Destination(0b100)),
                    operation: Operation::IncreaseA,
                    jump: None
                })
            ]),
            parsed
        )
    }
}
