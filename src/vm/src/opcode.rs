#[derive(Debug)]
pub struct PushOpCode<'a> {
    pub segment: &'a str,
    pub i: u16,
}

#[derive(Debug)]
pub struct PopOpCode<'a> {
    pub segment: &'a str,
    pub i: u16,
}

#[derive(Debug)]
pub enum OpCode<'a> {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Push(PushOpCode<'a>),
    Pop(PopOpCode<'a>),
}
