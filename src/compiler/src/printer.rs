use crate::token::*;
use std::fs;

pub fn print_tokens(tokens: &[Token], output_file: &str) {
    let mut output = String::new();

    output.push_str("<tokens>\n");
    for token in tokens {
        match token {
            Token::Identifier(id) => {
                output.push_str(&format!("<identifier> {} </identifier>\n", id));
            }
            Token::IntegerLiteral(literal) => {
                output.push_str(&format!(
                    "<integerConstant> {} </integerConstant>\n",
                    literal
                ));
            }
            Token::Keyword(_keyword, lexeme) => {
                output.push_str(&format!("<keyword> {} </keyword>\n", lexeme));
            }
            Token::StringLiteral(string) => {
                output.push_str(&format!("<stringConstant> {} </stringConstant>\n", string));
            }
            Token::Symbol(_symbol, lexeme) => {
                let lexeme = match lexeme.as_str() {
                    "<" => "&lt;",
                    ">" => "&gt;",
                    "\"" => "&quot;",
                    "&" => "&amp;",
                    _ => lexeme,
                };

                output.push_str(&format!("<symbol> {} </symbol>\n", lexeme));
            }
        }
    }
    output.push_str("</tokens>\n");

    fs::write(output_file, output).expect("Can not write to the tokens file");
}

pub fn print_ast(ast: &str, output_file: &str) {
    fs::write(output_file, ast).expect("Can not write to the AST file");
}
