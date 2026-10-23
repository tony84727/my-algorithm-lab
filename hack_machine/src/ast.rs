#[derive(Debug, PartialEq)]
pub enum AArgs<'p> {
    Address(u16),
    Symbol(&'p str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Jump {
    Noop,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Zero,
    One,
    NegativeOne,
    D,
    A,
    NotD,
    NotA,
    NegativeD,
    NegativeA,
    IncreaseD,
    IncreaseA,
    DecreaseD,
    DecreaseA,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    NegativeM,
    IncreaseM,
    DecreaseM,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[derive(Debug, PartialEq)]
pub struct Destination(pub u8);

#[derive(Debug, PartialEq)]
pub struct CArgs {
    pub destination: Option<Destination>,
    pub operation: Operation,
    pub jump: Option<Jump>,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'p> {
    AOp(AArgs<'p>),
    COp(CArgs),
    Label(&'p str),
    Comment(&'p str),
}

#[derive(Debug, PartialEq)]
pub struct Program<'p>(pub Vec<Expr<'p>>);
