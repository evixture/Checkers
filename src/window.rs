use crate::board::{map2d, Board, MoveMessage, Piece};
use iced::border::Radius;
use iced::widget::button::Style;
use iced::widget::{button, Button};
use iced::{color, Background, Border, Color, Element, Shadow, Theme};

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

pub fn update(_board: &mut Board, _msg: MoveMessage) {}

pub fn view(board: &Board) -> Element<MoveMessage> {
    let mut col: Vec<iced::Element<MoveMessage>> = Vec::with_capacity(Board::WIDTH);
    for y in 0..Board::WIDTH {
        let mut v: Vec<iced::Element<MoveMessage>> = Vec::with_capacity(Board::WIDTH);
        for x in 0..Board::WIDTH {
            v.push(
                Button::new(match board.board_arr[map2d(&x, &y, &Board::WIDTH)] {
                    Piece::None => "",
                    Piece::Red => "R",
                    Piece::Black => "B",
                })
                .width(100)
                .height(100)
                .style(if (y + x) % 2 == 0 {
                    style_white
                } else {
                    style_black
                })
                .into(),
            );
        }
        col.push(iced::widget::Row::from_vec(v).into());
    }
    iced::widget::Column::from_vec(col).into()
}

fn style_white(t: &Theme, s: button::Status) -> Style {
    Style {
        background: Option::from(Background::Color(color!(216, 183, 159))),
        text_color: Color::WHITE,
        border: Border {
            color: color!(100, 100, 100),
            width: 1f32,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
    }
}
fn style_black(t: &Theme, s: button::Status) -> Style {
    Style {
        background: Option::from(Background::Color(color!(97, 61, 51))),
        text_color: Color::BLACK,
        border: Border {
            color: color!(100, 100, 100),
            width: 1f32,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
    }
}
