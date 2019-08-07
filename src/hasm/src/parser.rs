use crate::instructions::*;

pub fn parse(source: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut ip: u16 = 0;

    for instruction in source.lines() {
        let instruction = instruction.trim().splitn(2, '/').next().unwrap().trim();
        if instruction.is_empty() {
            continue;
        };

        if instruction.starts_with('@') {
            let mnemonic = &instruction[1..];
            match mnemonic.parse::<u16>() {
                Ok(address) => instructions.push(Instruction::A(AInstruction::Literal(address))),
                Err(_) => instructions.push(Instruction::A(AInstruction::Mnemonic(mnemonic))),
            };

            ip += 1;
            continue;
        };

        if instruction.starts_with('(') {
            let mnemonic = instruction.trim_start_matches('(').trim_end_matches(')');
            instructions.push(Instruction::Label(LabelInstruction {
                name: mnemonic,
                ptr: ip,
            }));

            continue;
        }

        let dest = if instruction.contains('=') {
            let mnemonic = instruction.split('=').nth(0).unwrap();

            CDestinationInstruction {
                ram: mnemonic.contains('M'),
                a_register: mnemonic.contains('A'),
                d_register: mnemonic.contains('D'),
            }
        } else {
            CDestinationInstruction {
                ram: false,
                a_register: false,
                d_register: false,
            }
        };

        let jump = if instruction.contains(';') {
            let mnemonic = instruction.split(';').nth(1).unwrap();
            match mnemonic {
                "JGT" => CJumpInstruction {
                    greater_than: true,
                    equal: false,
                    lower_than: false,
                },
                "JEQ" => CJumpInstruction {
                    greater_than: false,
                    equal: true,
                    lower_than: false,
                },
                "JGE" => CJumpInstruction {
                    greater_than: true,
                    equal: true,
                    lower_than: false,
                },
                "JLT" => CJumpInstruction {
                    greater_than: false,
                    equal: false,
                    lower_than: true,
                },
                "JNE" => CJumpInstruction {
                    greater_than: true,
                    equal: false,
                    lower_than: true,
                },
                "JLE" => CJumpInstruction {
                    greater_than: false,
                    equal: true,
                    lower_than: true,
                },
                "JMP" => CJumpInstruction {
                    greater_than: true,
                    equal: true,
                    lower_than: true,
                },
                _ => panic!("Unknown destination instruction: {}", mnemonic),
            }
        } else {
            CJumpInstruction {
                greater_than: false,
                equal: false,
                lower_than: false,
            }
        };

        let mnemonic = if instruction.contains('=') {
            instruction.split('=').nth(1).unwrap()
        } else if instruction.contains(';') {
            instruction.split(';').nth(0).unwrap()
        } else {
            instruction
        };

        let comp = match mnemonic {
            "0" => CCompInstruction::Zero,
            "1" => CCompInstruction::One,
            "-1" => CCompInstruction::MinusOne,
            "D" => CCompInstruction::DRegister,
            "A" => CCompInstruction::ARegister,
            "M" => CCompInstruction::RAM,
            "!D" => CCompInstruction::NotDRegister,
            "!A" => CCompInstruction::NotARegister,
            "!M" => CCompInstruction::NotRAM,
            "-D" => CCompInstruction::MinusDRegister,
            "-A" => CCompInstruction::MinusARegister,
            "-M" => CCompInstruction::MinusRAM,
            "D+1" => CCompInstruction::DRegisterPlusOne,
            "A+1" => CCompInstruction::ARegisterPlusOne,
            "M+1" => CCompInstruction::RAMPlusOne,
            "D-1" => CCompInstruction::DRegisterMinusOne,
            "A-1" => CCompInstruction::ARegisterMinusOne,
            "M-1" => CCompInstruction::RAMMinusOne,
            "D+A" => CCompInstruction::DRegisterPlusARegister,
            "D+M" => CCompInstruction::DRegisterPlusRAM,
            "D-A" => CCompInstruction::DRegisterMinusARegister,
            "D-M" => CCompInstruction::DRegisterMinusRAM,
            "A-D" => CCompInstruction::ARegisterMinusDRegister,
            "M-D" => CCompInstruction::RAMMinusDRegister,
            "D&A" => CCompInstruction::DRegisterAndARegister,
            "D&M" => CCompInstruction::DRegisterAndRAM,
            "D|A" => CCompInstruction::DRegisterOrARegister,
            "D|M" => CCompInstruction::DRegisterOrRAM,
            _ => panic!("Unknown computation instruction: {}", mnemonic),
        };

        ip += 1;
        instructions.push(Instruction::C(CInstruction { dest, jump, comp }));
    }

    instructions
}
