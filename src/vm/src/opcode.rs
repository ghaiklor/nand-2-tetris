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
pub struct LabelOpCode<'a> {
    pub id: &'a str,
}

#[derive(Debug)]
pub struct GotoOpCode<'a> {
    pub id: &'a str,
}

#[derive(Debug)]
pub struct IfGotoOpCode<'a> {
    pub id: &'a str,
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
    Label(LabelOpCode<'a>),
    Goto(GotoOpCode<'a>),
    IfGoto(IfGotoOpCode<'a>),
}
