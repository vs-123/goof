use std::process;

use crate::tokens::{Token, TokenKind, Location};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub output_tokens: Vec<Token>,

    source_code_chars: Vec<char>,
    line_start_indexes: Vec<usize>,
    source_code_len: usize,
    source_code_lines: Vec<String>,

    current_char_index: usize,

    file_name: String,
}

impl Lexer {
    pub fn new(file_name: String, source_code: String) -> Self {
        Self { 
            output_tokens: Vec::new(), 

            source_code_chars: source_code.clone().chars().collect(), 
            line_start_indexes: vec![0],
            source_code_len: source_code.len(),
            source_code_lines: source_code.clone().lines().map(String::from).collect(),

            current_char_index: 0,

            file_name,
        }
    }

    pub fn lex(&mut self) {
        loop {
            if self.is_eof() { break; }

            match self.current_char() {
                '\n' => {self.line_start_indexes.push(self.current_char_index);},
                c if c.is_whitespace() => { self.next(); continue; }

                c if c.is_alphabetic() => {
                    let mut identifier_value = String::new();
                    let current_location = self.current_location();

                    while !self.is_eof() && self.current_char().is_alphanumeric() {
                        identifier_value.push(self.current_char());
                        self.next();
                    }

                    self.output_tokens.push(Token {
                        kind: TokenKind::Ident,
                        value: identifier_value,
                        location: current_location,
                    });
                    self.go_back();
                }

                c if c.is_numeric() => {
                    let mut numeric_value = String::new();
                    let current_location = self.current_location();

                    while !self.is_eof() && self.current_char().is_numeric() {
                        numeric_value.push(self.current_char());
                        self.next();
                    }

                    self.output_tokens.push(Token {
                        kind: TokenKind::Number,
                        value: numeric_value,
                        location: current_location,
                    });
                    self.go_back();
                }

                '"' => {
                    let mut string_value = String::new();
                    let current_location = self.current_location();

                    self.next();
                    while !self.is_eof() && self.current_char() != '"' {
                        string_value.push(self.current_char());
                        self.next();
                    }
                    if self.is_eof() {
                        self.go_back();
                        self.throw_err("Unended string.")
                    }

                    self.output_tokens.push(Token {
                        kind: TokenKind::String,
                        value: string_value,
                        location: current_location,
                    });
                }

                '(' => self.output_tokens.push(Token {
                    kind: TokenKind::OParen,
                    value: "(".to_string(),
                    location: self.current_location(),
                }),

                ')' => self.output_tokens.push(Token {
                    kind: TokenKind::CParen,
                    value: ")".to_string(),
                    location: self.current_location(),
                }),

                ',' => self.output_tokens.push(Token {
                    kind: TokenKind::Comma,
                    value: ",".to_string(),
                    location: self.current_location(),
                }),


                other => {
                    self.throw_err(format!(
                        "Unexpected character '{}' found (character code {})",
                        other, other as u8
                    ))
                }
            }
            self.next();
        }

        self.output_tokens.push(Token {
            kind: TokenKind::Eof,
            value: String::new(),
            location: self.current_location(),
        })
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.current_char_index == self.source_code_len
    }

    #[inline]
    fn next(&mut self) {
        self.current_char_index += 1;
    }

    #[inline]
    fn go_back(&mut self) {
        self.current_char_index -= 1;
    }

    #[inline]
    fn current_char(&self) -> char {
        self.source_code_chars[self.current_char_index]
    }

    #[inline]
    fn current_line_number(&self) -> usize {
        self.line_start_indexes.len()
    }

    #[inline]
    fn current_column(&self) -> usize {
        self.current_char_index - self.line_start_indexes.last().unwrap() + 1
    }

    #[inline]
    fn current_line(&self) -> String {
        self.source_code_lines[self.current_line_number()-1].clone()
    }

    #[inline]
    fn current_location(&self) -> Location {
        Location {
            file_name: self.file_name.clone(),
            column: self.current_column(),
            line_number: self.current_line_number(),
            line: self.current_line(),
        }
    }

    fn throw_err<T: std::fmt::Display>(&self, msg: T) {
        let current_line_number = self.current_line_number();
        let current_line_number_spaces = " ".repeat(((current_line_number as f64).log10()+1.).floor() as usize);

        println!("[Lexical Error]");
        println!("{}\n", msg);
        println!("[Where?]");
        println!(
            "In file '{}' at line {}, column {}\n",
            self.file_name, current_line_number, self.current_column()
        );
        println!("[Code]");
        println!(" {} |", current_line_number_spaces);
        println!(" {} | {}", current_line_number, self.current_line());
        println!(" {} | {}^", current_line_number_spaces, " ".repeat(self.current_column()-1));

        process::exit(69);
    }
}