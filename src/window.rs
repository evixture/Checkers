use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::Button;
use iced::{color, widget, Background, Border, Color, Element, Shadow, Theme};

use crate::board::{
    available_moves, available_moves_coord, map2d, map2d_coord, Board, BoardStateMsg, Piece,
};

pub fn update(b: &mut Board, msg: BoardStateMsg) {
    match msg {
        BoardStateMsg::Selection(x, y) => {
            if b.first.is_some() && b.second.is_none() {
                //deselect by selecting same spot
                let bf = b.first.unwrap();
                if b.first.unwrap() == (x, y) {
                    b.first = None;
                    return;
                    //println!("deselected first");
                } else {
                    //println!("selected second");
                    for av_move in available_moves_coord(b, bf) {
                        //println!("{:?} {}, {}", av_move, x, y);
                        if av_move == (x, y) {
                            //println!("matched move");
                            match b.turn {
                                Piece::Black => {
                                    b.turn = Piece::Red;
                                    b.board_arr[map2d_coord(&av_move, &Board::WIDTH)] =
                                        Piece::Black;
                                    println!("move black");
                                }
                                Piece::Red => {
                                    b.turn = Piece::Black;
                                    b.board_arr[map2d_coord(&av_move, &Board::WIDTH)] = Piece::Red;
                                    println!("move red");
                                }
                                _ => (),
                            }
                            b.board_arr[map2d_coord(&bf, &Board::WIDTH)] = Piece::None;
                            b.first = None;
                            b.second = None;
                        }
                    }
                }
            } else if b.first.is_none() {
                if b.board_arr[map2d(&x, &y, &Board::WIDTH)] == b.turn
                    && !available_moves(b, x, y).is_empty()
                {
                    b.first = Option::from((x, y));
                    //println!("selected first");
                }
            }
        }
    }
    //if msg == BoardStateMsg::Selection(a, b) {}
}

pub fn view(board: &Board) -> Element<BoardStateMsg> {
    let mut col: Vec<Element<BoardStateMsg>> = Vec::with_capacity(Board::WIDTH);
    for y in 0..Board::WIDTH {
        let mut v: Vec<Element<BoardStateMsg>> = Vec::with_capacity(Board::WIDTH);
        for x in 0..Board::WIDTH {
            v.push(
                Button::new(match board.board_arr[map2d(&x, &y, &Board::WIDTH)] {
                    Piece::None => iced::widget::image(""),
                    Piece::Red => widget::image("assets/red.png"),
                    Piece::Black => widget::image("assets/black.png"),
                })
                .on_press(BoardStateMsg::Selection(x, y))
                .width(100)
                .height(100)
                .style(
                    if match board.first {
                        //board.first.is_some() &&
                        Some((a, b)) => a == x && b == y,
                        _ => false,
                    } {
                        style_selected
                    } else if (y + x) % 2 == 0 {
                        style_white
                    } else {
                        style_black
                    },
                )
                .into(),
            );
        }
        col.push(widget::Row::from_vec(v).into());
    }
    widget::Column::from_vec(col).into()
}

//todo random colors
fn style_white(_t: &Theme, _s: Status) -> Style {
    Style {
        background: Option::from(Background::Color(match _s {
            Status::Hovered => color!(250, 250, 250),
            _ => color!(216, 183, 159),
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
        background: Option::from(Background::Color(match _s {
            Status::Hovered => color!(250, 250, 250),
            _ => color!(97, 61, 51),
        })),
        text_color: Color::BLACK,
        border: Border {
            color: color!(100, 100, 100),
            width: 1f32,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
    }
}

fn style_selected(_t: &Theme, _s: Status) -> Style {
    Style {
        background: Option::from(Background::Color(match _s {
            Status::Hovered => color!(254, 226, 82),
            _ => color!(254, 226, 82),
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
