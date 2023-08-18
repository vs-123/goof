use std::process;

use crate::ast::{Node, NodeKind};

pub struct Interpreter {
    input_nodes: Vec<Node>,
    input_nodes_len: usize,
    current_node_index: usize,
}

impl Interpreter {
    pub fn new(input_nodes: Vec<Node>) -> Self {
        Self {
            input_nodes_len: input_nodes.len(),
            input_nodes,
            current_node_index: 0,
        }
    }

    pub fn interpret(&mut self) {
        while self.current_node_index < self.input_nodes_len {
            let current_node = self.current_node();
            match current_node.kind {
                NodeKind::Func(_, _) => {
                    // dbg!(self.current_node_index, &current_node);
                    self.deal_specific_func(current_node);
                }

                other => {
                    self.throw_err(format!(
                        "Unimplemented NodeKind '{:?}'",
                        other
                    ));
                }
            }
            
            self.next();
        }
    }

    fn deal_specific_func(&mut self, current_func: Node) -> Option<Node> {
        if let NodeKind::Func(name, args) = current_func.kind {
            // Arguments after evaluating all function arguments
            // Will contain only NodeKind::String and NodeKind::Number
            // Leaving it equal to `args` for now

            let args = {
                let mut new_args: Vec<Node> = Vec::new();

                for arg in args.iter() {
                    let arg = arg.clone();
                    match arg.kind {
                        NodeKind::Func(_, _) => {
                            if let Some(val) = self.deal_specific_func(arg.clone()) {
                                new_args.push(val);
                            }
                        }

                        _ => new_args.push(arg.clone()),
                    }
                }

                new_args
            };

            match name.as_str() {
                "print" => {
                    for arg in args.iter() {
                        let arg = arg.clone();
                        match arg.kind {
                            NodeKind::String(string) => {
                                print!("{}", string);
                            }
                            
                            NodeKind::Number(num) => {
                                print!("{}", num);
                            }

                            NodeKind::Func(_, _) => unreachable!(),
                        }
                    }
                }

                "println" => {
                    for arg in args.iter() {
                        let arg = arg.clone();
                        match arg.kind {
                            NodeKind::String(string) => {
                                println!("{}", string);
                            }
                            
                            NodeKind::Number(num) => {
                                println!("{}", num);
                            }

                            NodeKind::Func(_, _) => unreachable!(),
                        }
                    }
                }

                "add" => {
                    if args.len() != 2 {
                        // dbg!(&args);
                        self.throw_err(format!(
                            "Function `add` requires only 2 (two) arguments, but got {}.\n\n{}{}",
                            args.len(),

                            "[Syntax]\n",
                            "add(Number, Number)",
                        ));
                    }

                    let operand1 = if let NodeKind::Number(operand1) = args[0].kind { operand1 } else {
                        self.throw_err("Expected type NodeKind::Number".to_string());
                        0.
                    };

                    let operand2 = if let NodeKind::Number(operand1) = args[1].kind { operand1 } else {
                        self.throw_err("Expected type NodeKind::Number".to_string());
                        0.
                    };

                    return Some(Node {
                        kind: NodeKind::Number(operand1 + operand2),
                        location: current_func.location,
                    });
                }

                other => {
                    self.throw_err(format!(
                        "Undefined function '{}'",
                        other
                    ))
                }
            }
        }

        None
    }

    fn next(&mut self) {
        self.current_node_index += 1;
    }

    #[inline]
    fn current_node(&self) -> Node {
        self.input_nodes[self.current_node_index].clone()
    }

    fn throw_err<T: std::fmt::Display>(&self, msg: T) {
        let location = self.current_node().location;
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