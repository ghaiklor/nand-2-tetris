use std::collections::HashMap;

#[derive(Debug)]
pub enum SymbolKind {
    Field,
    Static,
    Argument,
    Local,
}

#[derive(Debug)]
pub struct Symbol<'a> {
    pub r#type: &'a str,
    pub kind: &'a SymbolKind,
    pub index: u16,
}

#[derive(Default, Debug)]
pub struct SymbolTable<'a> {
    class_symbols: HashMap<&'a str, Symbol<'a>>,
    subroutine_symbols: HashMap<&'a str, Symbol<'a>>,
    static_index: u16,
    field_index: u16,
    argument_index: u16,
    local_index: u16,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            class_symbols: HashMap::new(),
            subroutine_symbols: HashMap::new(),
            static_index: 0,
            field_index: 0,
            argument_index: 0,
            local_index: 0,
        }
    }

    pub fn define(&mut self, name: &'a str, r#type: &'a str, kind: &'a SymbolKind) {
        let index = self.get_running_index(kind);
        let symbol = Symbol {
            r#type,
            kind,
            index,
        };

        match kind {
            SymbolKind::Static => {
                self.static_index += 1;
                self.class_symbols.insert(name, symbol);
            }
            SymbolKind::Field => {
                self.field_index += 1;
                self.class_symbols.insert(name, symbol);
            }
            SymbolKind::Argument => {
                self.argument_index += 1;
                self.subroutine_symbols.insert(name, symbol);
            }
            SymbolKind::Local => {
                self.local_index += 1;
                self.subroutine_symbols.insert(name, symbol);
            }
        };
    }

    fn get_running_index(&self, kind: &SymbolKind) -> u16 {
        match kind {
            SymbolKind::Argument => self.argument_index,
            SymbolKind::Field => self.field_index,
            SymbolKind::Static => self.static_index,
            SymbolKind::Local => self.local_index,
        }
    }

    pub fn get_variables_count(&self, kind: &SymbolKind) -> u16 {
        self.get_running_index(kind)
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.subroutine_symbols
            .get(name)
            .or_else(|| self.class_symbols.get(name))
    }

    pub fn get_kind_of(&self, name: &str) -> Option<&SymbolKind> {
        let symbol = self.get_symbol(name);

        match symbol {
            Option::Some(symbol) => Option::Some(symbol.kind),
            Option::None => Option::None,
        }
    }

    pub fn get_type_of(&self, name: &str) -> Option<&str> {
        let symbol = self.get_symbol(name);

        match symbol {
            Option::Some(symbol) => Option::Some(symbol.r#type),
            Option::None => Option::None,
        }
    }

    pub fn get_index_of(&self, name: &str) -> Option<u16> {
        let symbol = self.get_symbol(name);

        match symbol {
            Option::Some(symbol) => Option::Some(symbol.index),
            Option::None => Option::None,
        }
    }

    pub fn reset_subroutine_table(&mut self) {
        self.local_index = 0;
        self.argument_index = 0;
        self.subroutine_symbols.clear();
    }

    pub fn reset_class_table(&mut self) {
        self.field_index = 0;
        self.static_index = 0;
        self.class_symbols.clear();
    }

    pub fn kind_from_str(kind: &str) -> SymbolKind {
        match kind {
            "argument" => SymbolKind::Argument,
            "field" => SymbolKind::Field,
            "static" => SymbolKind::Static,
            "local" => SymbolKind::Local,
            _ => panic!("Unknown kind {}", kind),
        }
    }

    pub fn kind_to_str(kind: &SymbolKind) -> &'a str {
        match kind {
            SymbolKind::Argument => "argument",
            SymbolKind::Field => "field",
            SymbolKind::Static => "static",
            SymbolKind::Local => "local",
        }
    }
}
