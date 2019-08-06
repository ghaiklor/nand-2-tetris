use crate::parser::Instruction;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbols: HashMap<String, u16> = HashMap::new();
        symbols.insert(String::from("R0"), 0);
        symbols.insert(String::from("R1"), 1);
        symbols.insert(String::from("R2"), 2);
        symbols.insert(String::from("R3"), 3);
        symbols.insert(String::from("R4"), 4);
        symbols.insert(String::from("R5"), 5);
        symbols.insert(String::from("R6"), 6);
        symbols.insert(String::from("R7"), 7);
        symbols.insert(String::from("R8"), 8);
        symbols.insert(String::from("R9"), 9);
        symbols.insert(String::from("R10"), 10);
        symbols.insert(String::from("R11"), 11);
        symbols.insert(String::from("R12"), 12);
        symbols.insert(String::from("R13"), 13);
        symbols.insert(String::from("R14"), 14);
        symbols.insert(String::from("R15"), 15);
        symbols.insert(String::from("SCREEN"), 16384);
        symbols.insert(String::from("KBD"), 24576);
        symbols.insert(String::from("SP"), 0);
        symbols.insert(String::from("LCL"), 1);
        symbols.insert(String::from("ARG"), 2);
        symbols.insert(String::from("THIS"), 3);
        symbols.insert(String::from("THAT"), 4);

        SymbolTable { symbols }
    }

    pub fn has(&self, key: String) -> bool {
        self.symbols.contains_key(&key)
    }

    pub fn get(&self, key: String) -> &u16 {
        self.symbols.get(&key).unwrap()
    }

    pub fn set(&mut self, key: String, value: u16) {
        self.symbols.insert(key, value);
    }

    pub fn resolve(&mut self, instructions: &[Instruction]) -> &SymbolTable {
        for instruction in instructions {
            if let Instruction::Label(instruction) = instruction {
                self.set(String::from(&instruction.value), instruction.ptr);
            };
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

        assert!(symbol_table.has(String::from("R0")));
        assert!(!symbol_table.has(String::from("NOT_EXISTS")))
    }

    #[test]
    fn it_should_get() {
        let symbol_table = SymbolTable::new();

        assert_eq!(*symbol_table.get(String::from("R2")), 2);
    }

    #[test]
    fn it_should_set() {
        let mut symbol_table = SymbolTable::new();

        assert!(!symbol_table.has(String::from("LOOP")));
        symbol_table.set(String::from("LOOP"), 1234);
        assert_eq!(*symbol_table.get(String::from("LOOP")), 1234)
    }
}
