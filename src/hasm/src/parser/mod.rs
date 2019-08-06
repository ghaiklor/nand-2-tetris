#[derive(Debug)]
pub struct AInstruction<'a> {
    pub value: &'a str,
}

#[derive(Debug)]
pub struct CInstruction<'a> {
    pub dest: Option<&'a str>,
    pub comp: Option<&'a str>,
    pub jump: Option<&'a str>,
}

#[derive(Debug)]
pub struct LabelInstruction<'a> {
    pub value: &'a str,
    pub ptr: u16,
}

#[derive(Debug)]
pub enum Instruction<'a> {
    A(AInstruction<'a>),
    C(CInstruction<'a>),
    Label(LabelInstruction<'a>),
}

pub struct Parser<'a> {
    source: String,
    instructions: Vec<Instruction<'a>>,
    ip: u16,
}

impl<'a> Parser<'a> {
    pub fn new(source: &str) -> Parser {
        Parser {
            source: String::from(source),
            instructions: Vec::new(),
            ip: 0,
        }
    }

    pub fn parse(&mut self) -> &'a Vec<Instruction> {
        for instruction in self.source.lines() {
            let instruction = instruction.trim().splitn(2, '/').next().unwrap().trim();

            if instruction.is_empty() || instruction.starts_with("//") {
                continue;
            };

            if instruction.starts_with('@') {
                self.ip += 1;
                self.instructions.push(Instruction::A(AInstruction {
                    value: &instruction[1..],
                }));
                continue;
            };

            if instruction.starts_with('(') {
                self.instructions.push(Instruction::Label(LabelInstruction {
                    value: instruction.trim_start_matches('(').trim_end_matches(')'),
                    ptr: self.ip,
                }));
                continue;
            }

            let dest = if instruction.contains('=') {
                let parts: Vec<&str> = instruction.split('=').collect();
                Option::Some(parts[0])
            } else {
                Option::None
            };

            let jump = if instruction.contains(';') {
                let parts: Vec<&str> = instruction.split(';').collect();
                Option::Some(parts[1])
            } else {
                Option::None
            };

            let comp = if instruction.contains('=') {
                let parts: Vec<&str> = instruction.split('=').collect();
                Option::Some(parts[1])
            } else if instruction.contains(';') {
                let parts: Vec<&str> = instruction.split(';').collect();
                Option::Some(parts[0])
            } else {
                Option::Some(instruction)
            };

            self.ip += 1;
            self.instructions
                .push(Instruction::C(CInstruction { dest, jump, comp }));
        }

        &self.instructions
    }
}
