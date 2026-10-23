pub enum AArgs {
    Address(u16),
    Symbol(String),
}

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

pub enum Destination {
    Noop,
    A,
    M,
    D,
    AM,
    AD,
    MD,
    AMD,
}

pub struct CArgs {
    pub destination: Destination,
    pub operation: Operation,
    pub jump: Jump,
}

pub enum Expr {
    AOp(AArgs),
    COp(CArgs),
    Label,
}
