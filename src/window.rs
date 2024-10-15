use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::Button;
use iced::{color, widget, Background, Border, Color, Element, Shadow, Theme};

use crate::board::{map2d, Board, MoveMessage, Piece};

pub fn update(_board: &mut Board, _msg: MoveMessage) {
    match _msg {
        MoveMessage::Selection => println!("button pressed"),
        MoveMessage::None => (),
    }
}

pub fn view(board: &Board) -> Element<MoveMessage> {
    let mut col: Vec<Element<MoveMessage>> = Vec::with_capacity(Board::WIDTH);
    for y in 0..Board::WIDTH {
        let mut v: Vec<Element<MoveMessage>> = Vec::with_capacity(Board::WIDTH);
        for x in 0..Board::WIDTH {
            v.push(
                Button::new(match board.board_arr[map2d(&x, &y, &Board::WIDTH)] {
                    Piece::None => iced::widget::image(""),
                    Piece::Red => widget::image("assets/red.png"),
                    Piece::Black => widget::image("assets/black.png"),
                })
                .on_press(MoveMessage::Selection)
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
        col.push(widget::Row::from_vec(v).into());
    }
    widget::Column::from_vec(col).into()
}

fn style_white(_t: &Theme, _s: Status) -> Style {
    Style {
        background: Option::from(Background::Color(if _s == Status::Hovered {
            color!(250, 250, 250)
        } else {
            color!(216, 183, 159)
        })),
        text_color: Color::WHITE,
        border: Border {
            color: color!(100, 100, 100),
            width: 1f32,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
    }
}
fn style_black(_t: &Theme, _s: Status) -> Style {
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
