use crate::codegen::Codegen;
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

    fn keyword(&self, token: &Token) -> String {
        match token {
            Token::Keyword(_keyword, lexeme) => lexeme.to_owned(),
            _ => String::from(""),
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
        self.expect(TokenType::Keyword);

        if !self.eat(TokenType::Keyword) {
            self.eat(TokenType::Identifier);
        }

        while self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ';'
        {
            self.expect(TokenType::Identifier);

            if self.symbol(self.current_token) == ',' {
                self.expect(TokenType::Symbol);
            }
        }

        self.expect(TokenType::Symbol);
        self.ast.end_elem().unwrap();
    }

    fn subroutine_dec(&mut self) {
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
            if !self.eat(TokenType::Identifier) {
                self.eat(TokenType::Keyword);
            }

            if self.symbol(self.current_token) == ',' {
                self.eat(TokenType::Symbol);
            }
        }

        self.ast.end_elem().unwrap();
    }

    fn var_dec(&mut self) {
        self.ast.begin_elem("varDec").unwrap();
        self.expect(TokenType::Keyword);

        if !self.eat(TokenType::Keyword) {
            self.eat(TokenType::Identifier);
        }

        while self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ';'
        {
            if !self.eat(TokenType::Symbol) {
                self.eat(TokenType::Identifier);
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
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Identifier);

        if self.symbol(self.current_token) == '.' {
            self.expect(TokenType::Symbol);
            self.expect(TokenType::Identifier);
        }

        self.expect(TokenType::Symbol);
        self.expression_list();
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
        self.ast.begin_elem("whileStatement").unwrap();
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Symbol);
        self.expression();
        self.expect(TokenType::Symbol);
        self.expect(TokenType::Symbol);
        self.statements();
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
        self.ast.begin_elem("ifStatement").unwrap();
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Symbol);
        self.expression();
        self.expect(TokenType::Symbol);
        self.expect(TokenType::Symbol);
        self.statements();
        self.expect(TokenType::Symbol);

        if self.keyword(self.current_token) == "else" {
            self.expect(TokenType::Keyword);
            self.expect(TokenType::Symbol);
            self.statements();
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
            self.expect(TokenType::Symbol);
            self.term();
        }

        self.ast.end_elem().unwrap();
    }

    fn term(&mut self) {
        self.ast.begin_elem("term").unwrap();

        match self.current_token {
            Token::IntegerLiteral(_) => self.expect(TokenType::IntegerLiteral),
            Token::StringLiteral(_) => self.expect(TokenType::StringLiteral),
            Token::Keyword(_, _) => self.expect(TokenType::Keyword),
            Token::Symbol(symbol, _lexeme) => {
                if symbol == &Symbol::Minus || symbol == &Symbol::Tilde {
                    self.expect(TokenType::Symbol);
                    self.term();
                } else {
                    self.expect(TokenType::Symbol);
                    self.expression();
                    self.expect(TokenType::Symbol);
                }
            }
            Token::Identifier(_id) => {
                self.expect(TokenType::Identifier);

                if self.token_type(self.current_token) == TokenType::Symbol {
                    if self.symbol(self.current_token) == '[' {
                        self.expect(TokenType::Symbol);
                        self.expression();
                        self.expect(TokenType::Symbol);
                    } else if self.symbol(self.current_token) == '(' {
                        self.expect(TokenType::Symbol);
                        self.expression_list();
                        self.expect(TokenType::Symbol);
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

    fn expression_list(&mut self) {
        self.ast.begin_elem("expressionList").unwrap();

        if self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ')'
        {
            self.expression();

            while self.symbol(self.current_token) == ',' {
                self.expect(TokenType::Symbol);
                self.expression();
            }
        }

        self.ast.end_elem().unwrap();
    }
}
