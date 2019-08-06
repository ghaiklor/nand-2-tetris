use crate::parser::Instruction;
use crate::symbols::SymbolTable;

pub struct CodeGen;

impl CodeGen {
    pub fn gen(instructions: &[Instruction], symbol_table: &SymbolTable) -> String {
        let mut machine_code: String = String::new();

        for instruction in instructions {
            match instruction {
                Instruction::A(instruction) => {
                    let address: u16 = match instruction.value.parse() {
                        Result::Ok(address) => address,
                        Result::Err(_) => *symbol_table.symbols.get(&instruction.value).unwrap(),
                    };

                    machine_code.push_str(&format!("{:016b}\n", address));
                }
                Instruction::C(instruction) => {
                    let dest = match instruction.dest {
                        None => "000",
                        Some(i) => match i {
                            "M" => "001",
                            "D" => "010",
                            "A" => "100",
                            "MD" => "011",
                            "AM" => "101",
                            "AD" => "110",
                            "AMD" => "111",
                            &_ => panic!("Unknown destination: {}", i),
                        },
                    };

                    let comp = match instruction.comp {
                        None => panic!("Unknown computation: {}", instruction.comp.unwrap()),
                        Some(comp) => match comp {
                            "0" => "0101010",
                            "1" => "0111111",
                            "-1" => "0111010",
                            "D" => "0001100",
                            "A" => "0110000",
                            "!D" => "0001101",
                            "!A" => "0110001",
                            "-D" => "0001111",
                            "-A" => "0110011",
                            "D+1" => "0011111",
                            "A+1" => "0110111",
                            "D-1" => "0001110",
                            "A-1" => "0110010",
                            "D+A" => "0000010",
                            "D-A" => "0010011",
                            "A-D" => "0000111",
                            "D&A" => "0000000",
                            "D|A" => "0010101",
                            "M" => "1110000",
                            "!M" => "1110001",
                            "-M" => "1110011",
                            "M+1" => "1110111",
                            "M-1" => "1110010",
                            "D+M" => "1000010",
                            "D-M" => "1010011",
                            "M-D" => "1000111",
                            "D&M" => "1000000",
                            "D|M" => "1010101",
                            &_ => panic!("Unknown computation: {}", comp),
                        },
                    };

                    let jump = match instruction.jump {
                        Option::None => "000",
                        Option::Some(jump) => match jump {
                            "JGT" => "001",
                            "JEQ" => "010",
                            "JGE" => "011",
                            "JLT" => "100",
                            "JNE" => "101",
                            "JLE" => "110",
                            "JMP" => "111",
                            &_ => panic!("Unknown jump: {}", jump),
                        },
                    };

                    machine_code.push_str(&format!("111{}{}{}\n", comp, dest, jump));
                }
                Instruction::Label(_) => (),
            }
        }

        machine_code
    }
}
