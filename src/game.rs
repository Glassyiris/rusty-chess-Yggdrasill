use crate::message::Message;

pub trait Game {
    fn update(&mut self, message: Message);
}
