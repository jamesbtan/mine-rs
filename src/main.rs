mod mine;
use crate::mine::Board;

fn main() -> iced::Result {
    iced::run("Mine-rs", Board::update, Board::view)
}
