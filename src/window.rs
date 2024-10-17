use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::Button;
use iced::{color, widget, Background, Border, Color, Element, Shadow, Theme};

use crate::board::{available_moves, map2d, Board, BoardStateMsg, Piece};

pub fn update(b: &mut Board, msg: BoardStateMsg) {
    match msg {
        BoardStateMsg::Selection(x, y) => {
            if b.first.is_some() && b.second.is_none() {
                //deselect by selecting same spot
                if match b.first {
                    Some((a, b)) => a == x && b == y,
                    _ => false,
                } {
                    b.first = None;
                    println!("deselected first");
                } else {
                    //b.second = Option::from((x, y));
                    println!("selected second");
                    for av_move in available_moves(b, b.first.unwrap().0, b.first.unwrap().1) {
                        println!("{:?} {}, {}", av_move, x, y);
                        if av_move.0 == x && av_move.1 == y {
                            println!("matched move");
                            match b.turn {
                                Piece::Black => {
                                    b.turn = Piece::Red;
                                    b.board_arr[map2d(
                                        &b.first.unwrap().0,
                                        &b.first.unwrap().1,
                                        &Board::WIDTH,
                                    )] = Piece::None;
                                    b.board_arr[map2d(&av_move.0, &av_move.1, &Board::WIDTH)] =
                                        Piece::Black;
                                    b.first = None;
                                    b.second = None;
                                    println!("move black");
                                }
                                Piece::Red => {
                                    b.turn = Piece::Black;
                                    b.board_arr[map2d(
                                        &b.first.unwrap().0,
                                        &b.first.unwrap().1,
                                        &Board::WIDTH,
                                    )] = Piece::None;
                                    b.board_arr[map2d(&av_move.0, &av_move.1, &Board::WIDTH)] =
                                        Piece::Red;
                                    b.first = None;
                                    b.second = None;
                                    println!("move red");
                                }
                                _ => (),
                            }
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
