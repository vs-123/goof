use std::{env, fs};

pub mod tokens;
pub mod ast;

pub mod lexer;
pub mod parser;
pub mod interpreter;

use crate::{lexer::Lexer, parser::Parser, interpreter::Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("[Usage]");
        println!("{} <input_file>", args[0]);
        return;
    }

    let file_name = args[1].clone();

    match fs::read_to_string(&file_name) {
        Ok(source_code) => {
            let mut lexer = Lexer::new(file_name, source_code);
            lexer.lex();
            // dbg!(&lexer);

            let mut parser = Parser::new(lexer.output_tokens);
            parser.parse();
            // dbg!(&parser.output_nodes);

            let mut interpreter = Interpreter::new(parser.output_nodes);
            interpreter.interpret();
        }

        Err(err) => {
            println!("[Error]");
            println!("Could not open file '{}'\n", file_name);
            println!("[Reason]");
            println!("{}", err);
        }
    }
}
