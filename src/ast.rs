use crate::tokens::Location;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    Func(String, Vec<Node>), // <- Name, args

    Number(f32),
    String(String),
}