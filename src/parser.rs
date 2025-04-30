use std::process;

use crate::{ast::{Node, NodeKind}, tokens::{Token, TokenKind, Location}};

#[derive(Clone, Debug)]
pub struct Parser {
    pub output_nodes: Vec<Node>,

    input_tokens: Vec<Token>,
    input_tokens_len: usize,
    current_token_index: usize,
}

impl Parser {
    pub fn new(input_tokens: Vec<Token>) -> Self {
        Self {
            output_nodes: Vec::new(),
            input_tokens_len: input_tokens.len(),
            input_tokens,
            current_token_index: 0,
        }
    }

    pub fn parse(&mut self) {
        while self.current_token_index < self.input_tokens_len {
            let current_token = self.current_token();
            match current_token.kind {
                TokenKind::Ident => {
                    self.eat_func();
                }

                TokenKind::Eof => {
                    break;
                }

                other => { dbg!(other); }
            }
            self.next();
        }
    }

    fn eat_func(&mut self) {
        let name: String = self.current_token().value;
        let mut args: Vec<Node> = Vec::new();
        let location: Location = self.current_token().location;

        self.expect(TokenKind::OParen);
        self.next();

        self.next();
        loop {
            let current_token = self.current_token();
            match current_token.kind {
                TokenKind::String => {
                    args.push(Node {
                        kind: NodeKind::String(current_token.value),
                        location: current_token.location,
                    });
                }

                TokenKind::Number => {
                    args.push(Node {
                        kind: NodeKind::Number(current_token.value.parse().unwrap()),
                        location: current_token.location,
                    });
                }

                // Functions inside
                TokenKind::Ident => {
                    args.push(self.just_eat_func());
                }

                TokenKind::Comma => {
                    self.expect_one_of(&[TokenKind::String, TokenKind::Number]);
                }
    
                TokenKind::CParen => { break; }
    
                other => { dbg!(other);unimplemented!(); }
            }
            self.expect_one_of(&[TokenKind::String, TokenKind::Number, TokenKind::Comma, TokenKind::CParen]);
            self.next();
        }
        self.go_back();
        self.expect(TokenKind::CParen);
        self.next();

        self.output_nodes.push(Node {
            kind: NodeKind::Func(name, args),
            location,
        })
    }

    fn just_eat_func(&mut self) -> Node {
        let name: String = self.current_token().value;
        let mut args: Vec<Node> = Vec::new();
        let location: Location = self.current_token().location;

        self.expect(TokenKind::OParen);
        self.next();

        self.next();
        loop {
            let current_token = self.current_token();
            match current_token.kind {
                TokenKind::String => {
                    args.push(Node {
                        kind: NodeKind::String(current_token.value),
                        location: current_token.location,
                    });
                }

                TokenKind::Number => {
                    args.push(Node {
                        kind: NodeKind::Number(current_token.value.parse().unwrap()),
                        location: current_token.location,
                    });
                }

                // Functions inside
                TokenKind::Ident => {
                    args.push(self.just_eat_func());
                }

                TokenKind::Comma => {
                    self.expect_one_of(&[TokenKind::String, TokenKind::Number]);
                }
    
                TokenKind::CParen => { break; }
    
                other => { dbg!(other);unimplemented!(); }
            }
            self.expect_one_of(&[TokenKind::String, TokenKind::Number, TokenKind::Comma, TokenKind::CParen]);
            self.next();
        }
        self.go_back();
        self.expect(TokenKind::CParen);
        self.next();

        Node {
            kind: NodeKind::Func(name, args),
            location,
        }
    }

    fn expect(&self, expected_kind: TokenKind) {
        let next_token = self.peek();
        if next_token.kind != expected_kind {
            self.throw_err(format!(
                "Expected token {:?} after '{}', but found '{}' ({:?})",
                expected_kind, self.current_token().value, next_token.value, next_token.kind
            ));
        }
    }

    fn expect_one_of(&self, expected_kinds: &[TokenKind]) {
        let next_token = self.peek();
        if !expected_kinds.contains(&next_token.kind) {
            self.throw_err(format!(
                "Expected one of tokens {:?} after '{}', but found '{}' ({:?})",
                expected_kinds, self.current_token().value, next_token.value, next_token.kind
            ));
        }
    }

    fn peek(&self) -> Token {
        self.input_tokens[self.current_token_index+1].clone()
    }

    #[inline]
    fn current_token(&self) -> Token {
        self.input_tokens[self.current_token_index].clone()
    }

    fn next(&mut self) {
        self.current_token_index += 1;
    }

    fn go_back(&mut self) {
        self.current_token_index -= 1;
    }

    fn throw_err<T: std::fmt::Display>(&self, msg: T) {
        let location = self.current_token().location;
        let current_column = location.column;
        let current_line_number = location.line_number;
        let current_line_number_spaces = " ".repeat(((current_line_number as f64).log10()+1.).floor() as usize);
        let current_line = location.line;

        println!("[Parser Error]");
        println!("{}\n", msg);
        println!("[Where?]");
        println!(
            "In file '{}' at line {}, column {}\n",
            location.file_name, current_line_number, current_column
        );
        println!("[Code]");
        println!(" {} |", current_line_number_spaces);
        println!(" {} | {}", current_line_number, current_line);
        println!(" {} | {}^", current_line_number_spaces, " ".repeat(current_column-1));

        process::exit(69);
    }
}