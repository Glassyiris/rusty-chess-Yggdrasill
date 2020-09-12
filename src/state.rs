use crate::chess::Chess;

#[derive(Clone, Debug)]
pub struct State {
    chess: Option<Chess>,
    gray: i32,
    green: i32,
    pub is_root: Root,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Root {
    NONE,
    GRAY,
    GREEN,
}

impl State {
    pub(crate) fn set_chess(chess: Option<Chess>) -> Self {
        State {
            chess,
            gray: 0,
            green: 0,
            is_root: Root::NONE,
        }
    }

    pub(crate) fn default() -> Self {
        State {
            chess: None,
            gray: 0,
            green: 0,
            is_root: Root::NONE,
        }
    }
}
