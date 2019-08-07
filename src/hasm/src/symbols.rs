use crate::instructions::*;
use std::collections::HashMap;

fn init_symbol_table<'a>(symbol_table: &'a mut HashMap<&str, u16>) -> &'a HashMap<&'a str, u16> {
    symbol_table.insert("R0", 0);
    symbol_table.insert("R1", 1);
    symbol_table.insert("R2", 2);
    symbol_table.insert("R3", 3);
    symbol_table.insert("R4", 4);
    symbol_table.insert("R5", 5);
    symbol_table.insert("R6", 6);
    symbol_table.insert("R7", 7);
    symbol_table.insert("R8", 8);
    symbol_table.insert("R9", 9);
    symbol_table.insert("R10", 10);
    symbol_table.insert("R11", 11);
    symbol_table.insert("R12", 12);
    symbol_table.insert("R13", 13);
    symbol_table.insert("R14", 14);
    symbol_table.insert("R15", 15);
    symbol_table.insert("SCREEN", 16384);
    symbol_table.insert("KBD", 24576);
    symbol_table.insert("SP", 0);
    symbol_table.insert("LCL", 1);
    symbol_table.insert("ARG", 2);
    symbol_table.insert("THIS", 3);
    symbol_table.insert("THAT", 4);

    symbol_table
}

pub fn resolve<'a>(instructions: &'a [Instruction]) -> HashMap<&'a str, u16> {
    let mut symbols: HashMap<&str, u16> = HashMap::new();
    let mut next_address: u16 = 15;

    init_symbol_table(&mut symbols);

    for instruction in instructions {
        if let Instruction::Label(instruction) = instruction {
            symbols.insert(instruction.name, instruction.ptr);
        };
    }

    for instruction in instructions {
        if let Instruction::A(instruction) = instruction {
            match instruction {
                AInstruction::Literal(address) => address,
                AInstruction::Mnemonic(name) => symbols.entry(name).or_insert_with(|| {
                    next_address += 1;
                    next_address
                }),
            };
        }
    }

    symbols
}
