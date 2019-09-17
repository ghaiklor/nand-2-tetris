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
    output: XmlWriter<'a, File>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser {
            tokens,
            current_token: &tokens[0],
            index: 1,
            output: XmlWriter::new(tempfile().expect("Can not open AST file for writing")),
        }
    }

    pub fn parse(mut self) -> String {
        self.class();

        let mut ast_file = self.output.into_inner();
        let mut contents = String::new();
        ast_file.seek(SeekFrom::Start(0)).unwrap();
        ast_file
            .read_to_string(&mut contents)
            .expect("Can not read from AST file");

        contents
    }

    fn advance(&mut self) -> bool {
        if self.index < self.tokens.len() {
            self.write_token(self.current_token);
            self.current_token = &self.tokens[self.index];
            self.index += 1;
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

    fn write_token(&mut self, token: &Token) {
        match token {
            Token::Identifier(id) => {
                self.output.begin_elem("identifier").unwrap();
                self.output.text(&format!(" {} ", id)).unwrap();
                self.output.end_elem().unwrap();
            }
            Token::IntegerLiteral(literal) => {
                self.output.begin_elem("integerConstant").unwrap();
                self.output.text(&format!(" {} ", literal)).unwrap();
                self.output.end_elem().unwrap();
            }
            Token::Keyword(_keyword, lexeme) => {
                self.output.begin_elem("keyword").unwrap();
                self.output.text(&format!(" {} ", lexeme)).unwrap();
                self.output.end_elem().unwrap();
            }
            Token::StringLiteral(string) => {
                self.output.begin_elem("stringConstant").unwrap();
                self.output.text(&format!(" {} ", string)).unwrap();
                self.output.end_elem().unwrap();
            }
            Token::Symbol(_symbol, lexeme) => {
                self.output.begin_elem("symbol").unwrap();
                self.output.text(&format!(" {} ", lexeme)).unwrap();
                self.output.end_elem().unwrap();
            }
        }
    }

    fn class(&mut self) {
        self.output.begin_elem("class").unwrap();
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
        self.output.end_elem().unwrap();
    }

    fn class_var_dec(&mut self) {
        self.output.begin_elem("classVarDec").unwrap();
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
        self.output.end_elem().unwrap();
    }

    fn subroutine_dec(&mut self) {
        self.output.begin_elem("subroutineDec").unwrap();
        self.expect(TokenType::Keyword);

        if !self.eat(TokenType::Keyword) {
            self.eat(TokenType::Identifier);
        }

        self.expect(TokenType::Identifier);
        self.expect(TokenType::Symbol);

        self.parameter_list();
        self.expect(TokenType::Symbol);

        self.output.begin_elem("subroutineBody").unwrap();
        self.expect(TokenType::Symbol);

        while self.keyword(self.current_token) == "var" {
            self.var_dec();
        }

        self.statements();
        self.expect(TokenType::Symbol);
        self.output.end_elem().unwrap();
        self.output.end_elem().unwrap();
    }

    fn parameter_list(&mut self) {
        self.output.begin_elem("parameterList").unwrap();

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

        self.output.end_elem().unwrap();
    }

    fn var_dec(&mut self) {
        self.output.begin_elem("varDec").unwrap();
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

        self.output.end_elem().unwrap();
    }

    fn statements(&mut self) {
        self.output.begin_elem("statements").unwrap();

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

        self.output.end_elem().unwrap();
    }

    fn do_statement(&mut self) {
        self.output.begin_elem("doStatement").unwrap();
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
        self.output.end_elem().unwrap();
    }

    fn let_statement(&mut self) {
        self.output.begin_elem("letStatement").unwrap();
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
        self.output.end_elem().unwrap();
    }

    fn while_statement(&mut self) {
        self.output.begin_elem("whileStatement").unwrap();
        self.expect(TokenType::Keyword);
        self.expect(TokenType::Symbol);
        self.expression();
        self.expect(TokenType::Symbol);
        self.expect(TokenType::Symbol);
        self.statements();
        self.expect(TokenType::Symbol);
        self.output.end_elem().unwrap();
    }

    fn return_statement(&mut self) {
        self.output.begin_elem("returnStatement").unwrap();
        self.expect(TokenType::Keyword);

        if self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ';'
        {
            self.expression();
        }

        self.expect(TokenType::Symbol);
        self.output.end_elem().unwrap();
    }

    fn if_statement(&mut self) {
        self.output.begin_elem("ifStatement").unwrap();
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

        self.output.end_elem().unwrap();
    }

    fn expression(&mut self) {
        self.output.begin_elem("expression").unwrap();
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

        self.output.end_elem().unwrap();
    }

    fn term(&mut self) {
        self.output.begin_elem("term").unwrap();

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
        self.output.end_elem().unwrap();
    }

    fn expression_list(&mut self) {
        self.output.begin_elem("expressionList").unwrap();

        if self.token_type(self.current_token) != TokenType::Symbol
            || self.symbol(self.current_token) != ')'
        {
            self.expression();

            while self.symbol(self.current_token) == ',' {
                self.expect(TokenType::Symbol);
                self.expression();
            }
        }

        self.output.end_elem().unwrap();
    }
}
