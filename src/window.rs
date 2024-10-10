use crate::board::{Board, MoveMessage};
// iced::widget::button;

#[derive(Default)]
pub struct CheckersWindow {
    board: Board,
}

impl CheckersWindow {
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
    pub fn view(&self) -> iced::Element<MoveMessage> {
        //button("something").into()
        self.board.get_view()
        //self::Board::get_view(self.board)
    }
}
