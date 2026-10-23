use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending, newline, not_line_ending, space0},
    combinator::{map, map_res},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
};

use crate::ast::{AArgs, Expr, Program};

fn label<'p>(input: &'p str) -> IResult<&'p str, Expr<'p>> {
    map(delimited(tag("("), alphanumeric1, tag(")")), Expr::Label).parse(input)
}

fn a_command_address(input: &str) -> IResult<&str, AArgs<'_>> {
    map_res(digit1, |x: &str| x.parse::<u16>().map(AArgs::Address)).parse(input)
}

fn a_command_symbol(input: &str) -> IResult<&str, AArgs<'_>> {
    map(alphanumeric1, AArgs::Symbol).parse(input)
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

fn expr(input: &str) -> IResult<&'_ str, Expr<'_>> {
    preceded(many0(blank_line), alt((comment, label, a_command))).parse(input)
}

pub fn parse(input: &str) -> IResult<&'_ str, Program<'_>> {
    map(separated_list0(newline, expr), Program).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{AArgs, Expr},
        parser::expr,
    };

    #[test]
    fn test_label() {
        let (_remaining, parsed) = expr("(LOOP)").unwrap();
        assert_eq!(Expr::Label("LOOP"), parsed);
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
}
