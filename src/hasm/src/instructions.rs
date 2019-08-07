#[derive(Debug)]
pub enum AInstruction<'a> {
    Literal(u16),
    Mnemonic(&'a str),
}

#[derive(Debug)]
pub struct CDestinationInstruction {
    pub ram: bool,
    pub a_register: bool,
    pub d_register: bool,
}

#[derive(Debug)]
pub struct CJumpInstruction {
    pub greater_than: bool,
    pub equal: bool,
    pub lower_than: bool,
}

#[derive(Debug)]
pub enum CCompInstruction {
    Zero,
    One,
    MinusOne,
    DRegister,
    ARegister,
    RAM,
    NotDRegister,
    NotARegister,
    NotRAM,
    MinusDRegister,
    MinusARegister,
    MinusRAM,
    DRegisterPlusOne,
    ARegisterPlusOne,
    RAMPlusOne,
    DRegisterMinusOne,
    ARegisterMinusOne,
    RAMMinusOne,
    DRegisterPlusARegister,
    DRegisterPlusRAM,
    DRegisterMinusARegister,
    DRegisterMinusRAM,
    ARegisterMinusDRegister,
    RAMMinusDRegister,
    DRegisterAndARegister,
    DRegisterAndRAM,
    DRegisterOrARegister,
    DRegisterOrRAM,
}

#[derive(Debug)]
pub struct CInstruction {
    pub dest: CDestinationInstruction,
    pub comp: CCompInstruction,
    pub jump: CJumpInstruction,
}

#[derive(Debug)]
pub struct LabelInstruction<'a> {
    pub name: &'a str,
    pub ptr: u16,
}

#[derive(Debug)]
pub enum Instruction<'a> {
    A(AInstruction<'a>),
    C(CInstruction),
    Label(LabelInstruction<'a>),
}
