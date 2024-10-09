use crate::board::{Board, MoveMessage};
use iced::widget::button;
//use iced::{Center, Task};

#[derive(Default)]
pub struct CheckersWindow<'a> {
    board: Board<'a>,
}

impl<'a> CheckersWindow<'a> {
    pub fn new(self) -> Self {
        CheckersWindow {
            board: Board::new(),
        }
    }

    //iced::run expects a state and message from update function
    pub fn update(&mut self, _message: MoveMessage) {
        self.board.update();
    }

    //iced::run expects at least a state from view function
    pub fn view(&self) -> iced::Element<MoveMessage>
where {
        button("something").into()
    }
}
