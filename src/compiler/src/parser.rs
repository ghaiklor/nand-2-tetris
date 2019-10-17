use crate::codegen::Codegen;
use crate::codegen::VMArithmetic;
use crate::codegen::VMSegment;
use crate::symbol_table::SymbolKind;
use crate::symbol_table::SymbolTable;
use crate::token::*;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use tempfile::tempfile;
use xml_writer::XmlWriter;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current_token: &'a Token,
    index: usize,
    ast: XmlWriter<'a, File>,
    symbol_table: SymbolTable<'a>,
    codegen: Codegen,
    label_counter: u16,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser {
            tokens,
            current_token: &tokens[0],
            index: 1,
            ast: XmlWriter::new(tempfile().expect("Can not open AST file for writing")),
            symbol_table: SymbolTable::new(),
            codegen: Codegen::new(),
            label_counter: 0,
        }
    }

    pub fn parse(mut self) -> (String, String) {
        self.class();
        self.ast.write("\n").unwrap();

        let mut ast_file = self.ast.into_inner();
        let mut ast = String::new();
        ast_file.seek(SeekFrom::Start(0)).unwrap();
        ast_file
            .read_to_string(&mut ast)
            .expect("Can not read from AST file");

        (ast, self.codegen.vm_code)
    }

    fn advance(&mut self) -> bool {
        if self.index < self.tokens.len() {
            self.write_token_to_ast(self.current_token);
            self.current_token = &self.tokens[self.index];

            if self.index + 1 != self.tokens.len() {
                self.index += 1;
            }

            return true;
        }

        false
    }

    fn eat(&mut self, token_type: TokenType) -> bool {
        if self.token_type(self.current_token) == token_type {
            return self.advance();
        }

        false
    }

    fn expect(&mut self, token_type: TokenType) {
        if self.token_type(self.current_token) != token_type {
            panic!(
                "Expected {:?}, but got {:?}",
                token_type, self.current_token
            );
        }

        self.advance();
    }

    fn token_type(&self, token: &Token) -> TokenType {
        match token {
            Token::Identifier(_) => TokenType::Identifier,
            Token::IntegerLiteral(_) => TokenType::IntegerLiteral,
            Token::Keyword(_, _) => TokenType::Keyword,
            Token::StringLiteral(_) => TokenType::StringLiteral,
            Token::Symbol(_, _) => TokenType::Symbol,
        }
    }

    fn symbol(&self, token: &Token) -> char {
        match token {
            Token::Symbol(_symbol, character) => character.chars().next().unwrap(),
            _ => '\0',
        }
    }

    fn keyword(&self, token: &'a Token) -> &'a str {
        match token {
            Token::Keyword(_keyword, lexeme) => lexeme,
            _ => "",
        }
    }

    fn identifier(&self, token: &'a Token) -> &'a str {
        match token {
            Token::Identifier(id) => id,
            _ => "",
        }
    }

    fn write_token_to_ast(&mut self, token: &Token) {
        match token {
            Token::Identifier(id) => {
                self.ast
                    .elem_text("identifier", &format!(" {} ", id))
                    .unwrap();
            }
            Token::IntegerLiteral(literal) => {
                self.ast
                    .elem_text("integerConstant", &format!(" {} ", literal))
                    .unwrap();
            }
            Token::Keyword(_keyword, lexeme) => {
                self.ast
                    .elem_text("keyword", &format!(" {} ", lexeme))
                    .unwrap();
            }
            Token::StringLiteral(string) => {
                self.ast
                    .elem_text("stringConstant", &format!(" {} ", string))
                    .unwrap();
            }
            Token::Symbol(_symbol, lexeme) => {
                self.ast
                    .elem_text("symbol", &format!(" {} ", lexeme))
                    .unwrap();
            }
        }
    }

    fn class(&mut self) {
        self.symbol_table.reset_class_table();
        self.ast.begin_elem("class").unwrap();
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Identifier);
        self.expect(TokenType::Symbol);

        while self.keyword(self.current_token) == "static"
            || self.keyword(self.current_token) == "field"
        {
            self.class_var_dec();
        }

        while self.keyword(self.current_token) == "constructor"
            || self.keyword(self.current_token) == "function"
            || self.keyword(self.current_token) == "method"
        {
            self.subroutine_dec();
        }

        self.expect(TokenType::Symbol);
        self.ast.write("\n").unwrap();
        self.ast.end_elem().unwrap();
    }

    fn class_var_dec(&mut self) {
        self.ast.begin_elem("classVarDec").unwrap();

        let var_type: &str;
        let kind = match self.keyword(self.current_token) {
            "field" => &SymbolKind::Field,
            "static" => &SymbolKind::Static,
            _ => panic!("Unknown kind"),
        };
        self.expect(TokenType::Keyword);

        match self.current_token {
            Token::Keyword(_, lexeme) => {
                var_type = lexeme;
                self.expect(TokenType::Keyword);
            }
            Token::Identifier(id) => {
                var_type = id;
                self.expect(TokenType::Identifier);
            }
            _ => panic!("Unexpected token type"),
        }

        while self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ';'
        {
            if let Token::Identifier(name) = self.current_token {
                self.symbol_table.define(name, var_type, kind);
                self.expect(TokenType::Identifier);
            } else {
                panic!("Unexpected token type");
            }

            if self.symbol(self.current_token) == ',' {
                self.expect(TokenType::Symbol);
            }
        }

        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
    }

    fn subroutine_dec(&mut self) {
        self.symbol_table.reset_subroutine_table();
        self.ast.begin_elem("subroutineDec").unwrap();
        self.expect(TokenType::Keyword);

        if !self.eat(TokenType::Keyword) {
            self.eat(TokenType::Identifier);
        }

        self.expect(TokenType::Identifier);
        self.expect(TokenType::Symbol);

        self.parameter_list();
        self.expect(TokenType::Symbol);

        self.ast.begin_elem("subroutineBody").unwrap();
        self.expect(TokenType::Symbol);

        while self.keyword(self.current_token) == "var" {
            self.var_dec();
        }

        self.statements();
        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
        self.ast.end_elem().unwrap();
    }

    fn parameter_list(&mut self) {
        self.ast.begin_elem("parameterList").unwrap();

        while self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ')'
        {
            let arg_type: &str;
            let name: &str;
            match self.current_token {
                Token::Keyword(_, lexeme) => {
                    arg_type = lexeme;
                    self.expect(TokenType::Keyword);
                }
                Token::Identifier(id) => {
                    arg_type = id;
                    self.expect(TokenType::Identifier);
                }
                _ => panic!("Unknown argument type"),
            }

            if let Token::Identifier(id) = self.current_token {
                name = id;
                self.expect(TokenType::Identifier);
            } else {
                panic!("Unexpected token");
            }

            self.symbol_table
                .define(name, arg_type, &SymbolKind::Argument);

            if self.symbol(self.current_token) == ',' {
                self.eat(TokenType::Symbol);
            }
        }

        self.ast.end_elem().unwrap();
    }

    fn var_dec(&mut self) {
        self.ast.begin_elem("varDec").unwrap();
        self.expect(TokenType::Keyword);

        let var_type: &str;
        match self.current_token {
            Token::Keyword(_, lexeme) => {
                var_type = lexeme;
                self.expect(TokenType::Keyword);
            }
            Token::Identifier(id) => {
                var_type = id;
                self.expect(TokenType::Identifier);
            }
            _ => panic!("Unknown variable type"),
        }

        while self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ';'
        {
            if !self.eat(TokenType::Symbol) {
                if let Token::Identifier(id) = self.current_token {
                    self.symbol_table.define(id, var_type, &SymbolKind::Local);
                    self.expect(TokenType::Identifier);
                } else {
                    panic!("Unknown token type");
                }
            }
        }

        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
    }

    fn statements(&mut self) {
        self.ast.begin_elem("statements").unwrap();

        while let Token::Keyword(keyword, _) = self.current_token {
            match keyword {
                Keyword::Let => self.let_statement(),
                Keyword::If => self.if_statement(),
                Keyword::While => self.while_statement(),
                Keyword::Do => self.do_statement(),
                Keyword::Return => self.return_statement(),
                _ => panic!("Unknown statement, starting from {:?}", keyword),
            }
        }

        self.ast.end_elem().unwrap();
    }

    fn do_statement(&mut self) {
        self.ast.begin_elem("doStatement").unwrap();

        let mut fn_name: String;
        self.expect(TokenType::Keyword);

        fn_name = String::from(self.identifier(self.current_token));
        self.expect(TokenType::Identifier);

        if self.symbol(self.current_token) == '.' {
            self.expect(TokenType::Symbol);
            fn_name += &format!(".{}", self.identifier(self.current_token));
            self.expect(TokenType::Identifier);
        }

        self.expect(TokenType::Symbol);
        let args_count = self.expression_list();
        self.codegen.emit_call(&fn_name, args_count);
        self.expect(TokenType::Symbol);
        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
    }

    fn let_statement(&mut self) {
        self.ast.begin_elem("letStatement").unwrap();
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Identifier);

        if self.symbol(self.current_token) == '[' {
            self.expect(TokenType::Symbol);
            self.expression();
            self.expect(TokenType::Symbol);
        }

        self.expect(TokenType::Symbol);
        self.expression();
        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
    }

    fn while_statement(&mut self) {
        let label_l1 = format!("L{}", self.label_counter);
        self.label_counter += 1;
        let label_l2 = format!("L{}", self.label_counter);
        self.label_counter += 1;

        self.ast.begin_elem("whileStatement").unwrap();
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Symbol);
        self.codegen.emit_label(&label_l1);
        self.expression();
        self.codegen.emit_arithmetic(&VMArithmetic::Not);
        self.codegen.emit_if_goto(&label_l2);
        self.expect(TokenType::Symbol);
        self.expect(TokenType::Symbol);
        self.statements();
        self.codegen.emit_goto(&label_l1);
        self.codegen.emit_label(&label_l2);
        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
    }

    fn return_statement(&mut self) {
        self.ast.begin_elem("returnStatement").unwrap();
        self.expect(TokenType::Keyword);

        if self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ';'
        {
            self.expression();
        }

        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
    }

    fn if_statement(&mut self) {
        let label_l1 = format!("L{}", self.label_counter);
        self.label_counter += 1;
        let label_l2 = format!("L{}", self.label_counter);
        self.label_counter += 1;

        self.ast.begin_elem("ifStatement").unwrap();
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Symbol);
        self.expression();
        self.codegen.emit_arithmetic(&VMArithmetic::Not);
        self.codegen.emit_if_goto(&label_l1);
        self.expect(TokenType::Symbol);
        self.expect(TokenType::Symbol);
        self.statements();
        self.codegen.emit_goto(&label_l2);
        self.codegen.emit_label(&label_l1);
        self.expect(TokenType::Symbol);

        if self.keyword(self.current_token) == "else" {
            self.expect(TokenType::Keyword);
            self.expect(TokenType::Symbol);
            self.statements();
            self.codegen.emit_label(&label_l2);
            self.expect(TokenType::Symbol);
        }

        self.ast.end_elem().unwrap();
    }

    fn expression(&mut self) {
        self.ast.begin_elem("expression").unwrap();
        self.term();

        while self.symbol(self.current_token) == '+'
            || self.symbol(self.current_token) == '-'
            || self.symbol(self.current_token) == '*'
            || self.symbol(self.current_token) == '/'
            || self.symbol(self.current_token) == '&'
            || self.symbol(self.current_token) == '|'
            || self.symbol(self.current_token) == '<'
            || self.symbol(self.current_token) == '>'
            || self.symbol(self.current_token) == '='
        {
            let arithmetic = match self.symbol(self.current_token) {
                '+' => VMArithmetic::Add,
                '&' => VMArithmetic::And,
                '=' => VMArithmetic::Eq,
                '>' => VMArithmetic::Gt,
                '<' => VMArithmetic::Lt,
                '-' => VMArithmetic::Sub,
                '|' => VMArithmetic::Or,
                '/' | '*' => VMArithmetic::Add, // TODO: implement mul and div
                _ => panic!("not supported on vm level"),
            };

            self.expect(TokenType::Symbol);
            self.term();
            self.codegen.emit_arithmetic(&arithmetic);
        }

        self.ast.end_elem().unwrap();
    }

    fn term(&mut self) {
        self.ast.begin_elem("term").unwrap();

        match self.current_token {
            Token::IntegerLiteral(int) => {
                self.codegen.emit_push(&VMSegment::Constant, *int);
                self.expect(TokenType::IntegerLiteral)
            }
            Token::StringLiteral(_) => self.expect(TokenType::StringLiteral),
            Token::Keyword(_, _) => self.expect(TokenType::Keyword),
            Token::Symbol(symbol, _lexeme) => {
                if symbol == &Symbol::Minus || symbol == &Symbol::Tilde {
                    let arithmetic = match symbol {
                        Symbol::Minus => VMArithmetic::Neg,
                        Symbol::Tilde => VMArithmetic::Not,
                        _ => panic!("Not supported arithmetic"),
                    };

                    self.expect(TokenType::Symbol);
                    self.term();
                    self.codegen.emit_arithmetic(&arithmetic);
                } else {
                    self.expect(TokenType::Symbol);
                    self.expression();
                    self.expect(TokenType::Symbol);
                }
            }
            Token::Identifier(id) => {
                if let Option::Some(symbol) = self.symbol_table.get_symbol(id) {
                    let kind = match symbol.kind {
                        SymbolKind::Argument => &VMSegment::Argument,
                        SymbolKind::Field => &VMSegment::This,
                        SymbolKind::Local => &VMSegment::Local,
                        SymbolKind::Static => &VMSegment::Static,
                    };

                    let index = symbol.index;
                    self.codegen.emit_push(kind, index);
                }

                self.expect(TokenType::Identifier);

                if self.token_type(self.current_token) == TokenType::Symbol {
                    if self.symbol(self.current_token) == '[' {
                        self.expect(TokenType::Symbol);
                        self.expression();
                        self.expect(TokenType::Symbol);
                    } else if self.symbol(self.current_token) == '(' {
                        self.expect(TokenType::Symbol);
                        let args_count = self.expression_list();
                        self.expect(TokenType::Symbol);
                        self.codegen.emit_call(id, args_count);
                    } else if self.symbol(self.current_token) == '.' {
                        self.expect(TokenType::Symbol);
                        self.expect(TokenType::Identifier);
                        self.expect(TokenType::Symbol);
                        self.expression_list();
                        self.expect(TokenType::Symbol);
                    }
                }
            }
        }
        self.ast.end_elem().unwrap();
    }

    fn expression_list(&mut self) -> u16 {
        self.ast.begin_elem("expressionList").unwrap();

        let mut expressions_count = 0;
        if self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ')'
        {
            self.expression();
            expressions_count += 1;

            while self.symbol(self.current_token) == ',' {
                self.expect(TokenType::Symbol);
                self.expression();
            }
        }

        self.ast.end_elem().unwrap();
        expressions_count
    }
}
