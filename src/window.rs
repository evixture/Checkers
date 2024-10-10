use crate::board::{map2d, piece_to_string, Board, MoveMessage, Piece};
use iced::widget::{button, Button, Container, Row};
use iced::Element;
// iced::widget::button;

// #[derive(Default)]
// pub struct CheckersWindow {
//     board: Board,
// }
//
// impl CheckersWindow {
//     pub fn new(self) -> Self {
//         CheckersWindow {
//             board: Board::new(),
//         }
//     }
//
//     //iced::run expects a state and message from update function
//     pub fn update(&mut self, _message: MoveMessage) {
//         self.board.update();
//     }
//
//     //iced::run expects at least a state from view function
//     pub fn view(&self) -> iced::Element<MoveMessage> {
//         //button("something").into()
//         self.board.get_view()
//         //self::Board::get_view(self.board)
//     }
// }

pub fn update(board: &mut Board, msg: MoveMessage) {}

pub fn view(board: &Board) -> Element<MoveMessage> {
    let mut col: Vec<iced::Element<MoveMessage>> = Vec::with_capacity(Board::WIDTH);
    let mut v: Vec<iced::Element<MoveMessage>> = Vec::with_capacity(Board::WIDTH);

    for y in 0..Board::WIDTH {
        for x in 0..Board::WIDTH {
            let str: String;
            match board.board_arr[map2d(&x, &y, &Board::WIDTH)] {
                Piece::None => str = "X".to_string(),
                Piece::Red => str = "R".to_string(),
                Piece::Black => str = "B".to_string(),
            };
            v.push(Button::new("X").into());
            //r.push(button(piece_to_string()));
        }
        col.push(iced::widget::Row::from_vec(v).into());
    }
    iced::widget::Column::from_vec(col).into()
}
