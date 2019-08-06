use crate::parser::Instruction;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct SymbolTable<'a> {
    pub symbols: HashMap<&'a str, u16>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        let mut symbols: HashMap<&'a str, u16> = HashMap::new();
        symbols.insert("R0", 0);
        symbols.insert("R1", 1);
        symbols.insert("R2", 2);
        symbols.insert("R3", 3);
        symbols.insert("R4", 4);
        symbols.insert("R5", 5);
        symbols.insert("R6", 6);
        symbols.insert("R7", 7);
        symbols.insert("R8", 8);
        symbols.insert("R9", 9);
        symbols.insert("R10", 10);
        symbols.insert("R11", 11);
        symbols.insert("R12", 12);
        symbols.insert("R13", 13);
        symbols.insert("R14", 14);
        symbols.insert("R15", 15);
        symbols.insert("SCREEN", 16384);
        symbols.insert("KBD", 24576);
        symbols.insert("SP", 0);
        symbols.insert("LCL", 1);
        symbols.insert("ARG", 2);
        symbols.insert("THIS", 3);
        symbols.insert("THAT", 4);

        SymbolTable { symbols }
    }

    pub fn has(&self, key: &str) -> bool {
        self.symbols.contains_key(key)
    }

    pub fn get(&self, key: &str) -> &u16 {
        self.symbols.get(key).unwrap()
    }

    pub fn set(&mut self, key: &'a str, value: u16) {
        self.symbols.insert(key, value);
    }

    pub fn resolve(&mut self, instructions: &'a [Instruction]) -> &SymbolTable {
        for instruction in instructions {
            if let Instruction::Label(instruction) = instruction {
                self.set(instruction.value, instruction.ptr);
            };
        }

        for instruction in instructions {
            let mut next_address: u16 = 15;

            if let Instruction::A(instruction) = instruction {
                match instruction.value.parse::<u16>() {
                    Result::Ok(address) => address,
                    Result::Err(_) => *self.symbols.entry(instruction.value).or_insert_with(|| {
                        next_address += 1;
                        next_address
                    }),
                };
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_have() {
        let symbol_table = SymbolTable::new();

        assert!(symbol_table.has("R0"));
        assert!(!symbol_table.has("NOT_EXISTS"));
    }

    #[test]
    fn it_should_get() {
        let symbol_table = SymbolTable::new();

        assert_eq!(*symbol_table.get("R2"), 2);
    }

    #[test]
    fn it_should_set() {
        let mut symbol_table = SymbolTable::new();

        assert!(!symbol_table.has("LOOP"));
        symbol_table.set("LOOP", 1234);
        assert_eq!(*symbol_table.get("LOOP"), 1234)
    }
}
