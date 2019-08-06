#[derive(Debug)]
pub struct AInstruction {
    pub value: String,
}

#[derive(Debug)]
pub struct CInstruction {
    pub dest: Option<String>,
    pub comp: Option<String>,
    pub jump: Option<String>,
}

#[derive(Debug)]
pub struct LabelInstruction {
    pub value: String,
    pub ptr: u16,
}

#[derive(Debug)]
pub enum Instruction {
    A(AInstruction),
    C(CInstruction),
    Label(LabelInstruction),
}

pub struct Parser {
    source: String,
    instructions: Vec<Instruction>,
    ip: u16,
}

impl Parser {
    pub fn new(source: &str) -> Parser {
        Parser {
            source: String::from(source),
            instructions: Vec::new(),
            ip: 0,
        }
    }

    pub fn parse(&mut self) -> &Vec<Instruction> {
        for instruction in self.source.lines() {
            let instruction = instruction.trim().splitn(2, '/').next().unwrap().trim();

            if instruction.is_empty() || instruction.starts_with("//") {
                continue;
            };

            if instruction.starts_with('@') {
                self.ip += 1;
                self.instructions.push(Instruction::A(AInstruction {
                    value: String::from(&instruction[1..]),
                }));
                continue;
            };

            if instruction.starts_with('(') {
                self.instructions.push(Instruction::Label(LabelInstruction {
                    value: String::from(instruction.trim_start_matches('(').trim_end_matches(')')),
                    ptr: self.ip,
                }));
                continue;
            }

            let dest = if instruction.contains('=') {
                let parts: Vec<&str> = instruction.split('=').collect();
                Option::Some(String::from(parts[0]))
            } else {
                Option::None
            };

            let jump = if instruction.contains(';') {
                let parts: Vec<&str> = instruction.split(';').collect();
                Option::Some(String::from(parts[1]))
            } else {
                Option::None
            };

            let comp = if instruction.contains('=') {
                let parts: Vec<&str> = instruction.split('=').collect();
                Option::Some(String::from(parts[1]))
            } else if instruction.contains(';') {
                let parts: Vec<&str> = instruction.split(';').collect();
                Option::Some(String::from(parts[0]))
            } else {
                Option::Some(String::from(instruction))
            };

            self.ip += 1;
            self.instructions
                .push(Instruction::C(CInstruction { dest, jump, comp }));
        }

        &self.instructions
    }
}
