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
pub struct FunctionOpCode<'a> {
    pub id: &'a str,
    pub vars_count: u16,
}

#[derive(Debug)]
pub struct CallOpCode<'a> {
    pub id: &'a str,
    pub args_count: u16,
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
    Return,
    Push(PushOpCode<'a>),
    Pop(PopOpCode<'a>),
    Label(LabelOpCode<'a>),
    Goto(GotoOpCode<'a>),
    IfGoto(IfGotoOpCode<'a>),
    Function(FunctionOpCode<'a>),
    Call(CallOpCode<'a>),
}
