use crate::instructions::*;
use std::collections::HashMap;

#[allow(clippy::implicit_hasher)]
pub fn codegen(instructions: &[Instruction], symbol_table: &HashMap<&str, u16>) -> String {
    let mut machine_code = String::new();

    for instruction in instructions {
        match instruction {
            Instruction::A(instruction) => {
                let address: u16 = match instruction {
                    AInstruction::Literal(address) => *address,
                    AInstruction::Mnemonic(name) => *symbol_table
                        .get(name)
                        .unwrap_or_else(|| panic!("Unresolved symbol: {}", name)),
                };

                machine_code.push_str(&format!("{:016b}\n", address));
            }

            Instruction::C(instruction) => {
                let dest = &instruction.dest;
                let dest: &str = &format!(
                    "{}{}{}",
                    dest.a_register as u8, dest.d_register as u8, dest.ram as u8
                );

                let jump = &instruction.jump;
                let jump: &str = &format!(
                    "{}{}{}",
                    jump.lower_than as u8, jump.equal as u8, jump.greater_than as u8
                );

                let comp = match instruction.comp {
                    CCompInstruction::Zero => "0101010",
                    CCompInstruction::One => "0111111",
                    CCompInstruction::MinusOne => "0111010",
                    CCompInstruction::DRegister => "0001100",
                    CCompInstruction::ARegister => "0110000",
                    CCompInstruction::NotDRegister => "0001101",
                    CCompInstruction::NotARegister => "0110001",
                    CCompInstruction::MinusDRegister => "0001111",
                    CCompInstruction::MinusARegister => "0110011",
                    CCompInstruction::DRegisterPlusOne => "0011111",
                    CCompInstruction::ARegisterPlusOne => "0110111",
                    CCompInstruction::DRegisterMinusOne => "0001110",
                    CCompInstruction::ARegisterMinusOne => "0110010",
                    CCompInstruction::DRegisterPlusARegister => "0000010",
                    CCompInstruction::DRegisterMinusARegister => "0010011",
                    CCompInstruction::ARegisterMinusDRegister => "0000111",
                    CCompInstruction::DRegisterAndARegister => "0000000",
                    CCompInstruction::DRegisterOrARegister => "0010101",
                    CCompInstruction::RAM => "1110000",
                    CCompInstruction::NotRAM => "1110001",
                    CCompInstruction::MinusRAM => "1110011",
                    CCompInstruction::RAMPlusOne => "1110111",
                    CCompInstruction::RAMMinusOne => "1110010",
                    CCompInstruction::DRegisterPlusRAM => "1000010",
                    CCompInstruction::DRegisterMinusRAM => "1010011",
                    CCompInstruction::RAMMinusDRegister => "1000111",
                    CCompInstruction::DRegisterAndRAM => "1000000",
                    CCompInstruction::DRegisterOrRAM => "1010101",
                };

                machine_code.push_str(&format!("111{}{}{}\n", comp, dest, jump));
            }

            Instruction::Label(_) => (),
        }
    }

    machine_code
}
