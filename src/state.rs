use crate::chess::{Chess, Position, Camp};

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
    pub(crate) fn set_chess(&mut self, chess: Option<Chess>) {
        if self.can_set() {
            self.chess = chess;
        } else {
            return;
        }

        //TODO: update,push


    }

    pub(crate) fn default() -> Self {
        State {
            chess: None,
            gray: 0,
            green: 0,
            is_root: Root::NONE,
        }
    }

    pub(crate) fn drop(&mut self) {
        let camp = self.chess.as_ref().unwrap().camp.clone();
        self.chess = None;
        //TODO: update
    }

    fn can_set(&self) -> bool {
        let mut a = false;
        if let Some(ches) = self.chess.clone() {
            match ches.camp {
                Camp::GREEN if self.green != 0 => a = true,
                Camp::GRAY if self.gray != 0 => a = true,
                _ => {}
            }
        }
        a && self.chess.is_none()
    }
}
