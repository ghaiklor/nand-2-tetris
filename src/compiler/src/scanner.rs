use crate::token::*;

pub struct Scanner {
    index: usize,
    source: String,
}

impl Scanner {
    pub fn new(source_code: &str) -> Scanner {
        Scanner {
            index: 0,
            source: String::from(source_code),
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let length = self.source.chars().count();

        while self.index < length {
            let character = self
                .source
                .get(self.index..=self.index)
                .unwrap()
                .chars()
                .next()
                .unwrap();

            let next_character = self
                .source
                .get(self.index + 1..=self.index + 1)
                .unwrap_or_else(|| "")
                .chars()
                .next()
                .unwrap_or_else(|| '\0');

            if character == '/' && next_character == '/' {
                self.skip_comments();
                continue;
            }

            if character == '/' && next_character == '*' {
                self.skip_comments();
                continue;
            }

            if character.is_whitespace() {
                self.index += 1;
                continue;
            }

            if character.is_numeric() {
                tokens.push(self.scan_integer_literal());
                continue;
            }

            if character == '"' {
                self.index += 1;
                tokens.push(self.scan_string_literal());
                continue;
            }

            if character.is_alphanumeric() {
                tokens.push(self.scan_identifier_or_keyword());
                continue;
            }

            tokens.push(self.scan_symbol());
        }

        tokens
    }

    fn skip_comments(&mut self) {
        let length = self.source.chars().count();
        let mut character = self
            .source
            .get(self.index..=self.index)
            .unwrap()
            .chars()
            .next()
            .unwrap();

        let mut next_character = self
            .source
            .get(self.index + 1..=self.index + 1)
            .unwrap()
            .chars()
            .next()
            .unwrap();

        if character == '/' && next_character == '/' {
            while character != '\n' && self.index < length {
                self.index += 1;
                character = self
                    .source
                    .get(self.index..=self.index)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
            }

            self.index += 1;
        }

        if character == '/' && next_character == '*' {
            self.index += 2;

            while character != '*' && next_character != '/' && self.index < length {
                self.index += 1;
                character = self
                    .source
                    .get(self.index..=self.index)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();

                next_character = self
                    .source
                    .get(self.index + 1..=self.index + 1)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
            }

            self.index += 2;
        }
    }

    fn scan_symbol(&mut self) -> Token {
        let character = self
            .source
            .get(self.index..=self.index)
            .unwrap()
            .chars()
            .next()
            .unwrap();
        self.index += 1;

        match character {
            '{' => Token::Symbol(Symbol::LeftCurlyBraces, String::from("{")),
            '}' => Token::Symbol(Symbol::RightCurlyBraces, String::from("}")),
            '(' => Token::Symbol(Symbol::LeftParenthesis, String::from("(")),
            ')' => Token::Symbol(Symbol::RightParenthesis, String::from(")")),
            '[' => Token::Symbol(Symbol::LeftSquareBrackets, String::from("[")),
            ']' => Token::Symbol(Symbol::RightSquareBrackets, String::from("]")),
            '.' => Token::Symbol(Symbol::Dot, String::from(".")),
            ',' => Token::Symbol(Symbol::Comma, String::from(",")),
            ';' => Token::Symbol(Symbol::Semicolon, String::from(";")),
            '+' => Token::Symbol(Symbol::Plus, String::from("+")),
            '-' => Token::Symbol(Symbol::Minus, String::from("-")),
            '*' => Token::Symbol(Symbol::Asterisk, String::from("*")),
            '/' => Token::Symbol(Symbol::Slash, String::from("/")),
            '&' => Token::Symbol(Symbol::Ampersand, String::from("&")),
            '|' => Token::Symbol(Symbol::VerticalBar, String::from("|")),
            '<' => Token::Symbol(Symbol::LessThan, String::from("<")),
            '>' => Token::Symbol(Symbol::GreaterThan, String::from(">")),
            '=' => Token::Symbol(Symbol::Equal, String::from("=")),
            '~' => Token::Symbol(Symbol::Tilde, String::from("~")),
            _ => panic!("Unknown character: {}", character),
        }
    }

    fn scan_integer_literal(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self
            .source
            .get(self.index..=self.index)
            .unwrap()
            .chars()
            .next()
            .unwrap();

        while character.is_numeric() && self.index < length {
            buffer.push(character);
            self.index += 1;
            character = self
                .source
                .get(self.index..=self.index)
                .unwrap()
                .chars()
                .next()
                .unwrap();
        }

        let integer: u16 = buffer.parse().expect("Integer literal expected");
        Token::IntegerLiteral(integer)
    }

    fn scan_string_literal(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self
            .source
            .get(self.index..=self.index)
            .unwrap()
            .chars()
            .next()
            .unwrap();

        while character != '"' && self.index < length {
            buffer.push(character);
            self.index += 1;
            character = self
                .source
                .get(self.index..=self.index)
                .unwrap()
                .chars()
                .next()
                .unwrap();
        }

        self.index += 1;
        Token::StringLiteral(buffer)
    }

    fn scan_identifier_or_keyword(&mut self) -> Token {
        let length = self.source.chars().count();
        let mut buffer = String::new();
        let mut character = self
            .source
            .get(self.index..=self.index)
            .unwrap()
            .chars()
            .next()
            .unwrap();

        while character.is_alphanumeric() && self.index < length {
            buffer.push(character);
            self.index += 1;
            character = self
                .source
                .get(self.index..=self.index)
                .unwrap()
                .chars()
                .next()
                .unwrap();
        }

        match buffer.as_str() {
            "class" => Token::Keyword(Keyword::Class, buffer),
            "constructor" => Token::Keyword(Keyword::Constructor, buffer),
            "function" => Token::Keyword(Keyword::Function, buffer),
            "method" => Token::Keyword(Keyword::Method, buffer),
            "field" => Token::Keyword(Keyword::Field, buffer),
            "static" => Token::Keyword(Keyword::Static, buffer),
            "var" => Token::Keyword(Keyword::Var, buffer),
            "int" => Token::Keyword(Keyword::Int, buffer),
            "char" => Token::Keyword(Keyword::Char, buffer),
            "boolean" => Token::Keyword(Keyword::Boolean, buffer),
            "void" => Token::Keyword(Keyword::Void, buffer),
            "true" => Token::Keyword(Keyword::True, buffer),
            "false" => Token::Keyword(Keyword::False, buffer),
            "null" => Token::Keyword(Keyword::Null, buffer),
            "this" => Token::Keyword(Keyword::This, buffer),
            "let" => Token::Keyword(Keyword::Let, buffer),
            "do" => Token::Keyword(Keyword::Do, buffer),
            "if" => Token::Keyword(Keyword::If, buffer),
            "else" => Token::Keyword(Keyword::Else, buffer),
            "while" => Token::Keyword(Keyword::While, buffer),
            "return" => Token::Keyword(Keyword::Return, buffer),
            _ => Token::Identifier(buffer),
        }
    }
}
