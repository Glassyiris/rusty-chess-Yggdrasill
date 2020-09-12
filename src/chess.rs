#[derive(Debug, Clone)]
pub enum ChessKind {
    LEAF,
    TRUNK,
    BRANCH,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(Clone, Debug)]
pub struct Chess {
    kind: ChessKind,
    position: Position,
    camp: Camp,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Camp {
    GRAY,
    GREEN,
}

impl Chess {
    pub fn new(position: Position, kind: ChessKind, camp: Camp) -> Self {
        Chess {
            kind,
            position,
            camp,
        }
    }
}
