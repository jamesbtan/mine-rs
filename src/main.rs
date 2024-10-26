mod mine;
use iced::{time::{self, Duration}, Subscription};
use crate::mine::{Board, Message};

fn main() -> iced::Result {
    iced::application("Mine-rs", Board::update, Board::view).subscription(timer).run()
}

fn timer(_state: &Board) -> Subscription<Message> {
    time::every(Duration::new(1, 0)).map(|_| Message::TimeTick)
}
