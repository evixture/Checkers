use crate::board::*;
use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::Button;
use iced::{color, widget, Background, Border, Color, Element, Shadow, Theme};

pub fn update(board: &mut Board, msg: BoardStateMsg) {
    match msg {
        BoardStateMsg::Selection(x, y) => {
            if board.first.is_some() {
                //deselect by selecting same spot
                let bf = board.first.unwrap();
                let mr = available_moves_coord(board, bf);
                if board.first.unwrap() == (x, y) {
                    board.first = None;
                    return;
                    //println!("deselected first");
                } else {
                    //println!("selected second");
                    for ma in mr {
                        match ma {
                            MoveAction::Move((a, b)) => {
                                if (a, b) == (x, y) {
                                    board.board_arr[map2d_coord(&(a, b))] = board.turn.clone();
                                    board.board_arr[map2d_coord(&board.first.unwrap())] =
                                        Piece::None;
                                    if board.turn == Piece::Black {
                                        board.turn = Piece::Red
                                    } else {
                                        board.turn = Piece::Black;
                                    }
                                    board.first = None;
                                }
                            }
                            MoveAction::Capture(((a, b), (c, d))) => {
                                if (a, b) == (x, y) {
                                    board.board_arr[map2d_coord(&(a, b))] = board.turn.clone();
                                    board.board_arr[map2d_coord(&board.first.unwrap())] =
                                        Piece::None;
                                    board.board_arr[map2d_coord(&(c, d))] = Piece::None;
                                    if board.turn == Piece::Black {
                                        board.turn = Piece::Red
                                    } else {
                                        board.turn = Piece::Black;
                                    }
                                    board.first = None;
                                }
                            }
                        }
                    }

                    // for av_move in v1.0 {
                    //     //println!("{:?} {}, {}", av_move, x, y);
                    //     if av_move == (x, y) {
                    //         //println!("matched move");
                    //         match b.turn {
                    //             Piece::Black => {
                    //                 b.turn = Piece::Red;
                    //                 b.board_arr[map2d_coord(&av_move)] = Piece::Black;
                    //                 println!("move black");
                    //             }
                    //             Piece::Red => {
                    //                 b.turn = Piece::Black;
                    //                 b.board_arr[map2d_coord(&av_move)] = Piece::Red;
                    //                 println!("move red");
                    //             }
                    //             _ => (),
                    //         }
                    //         //remove all captured
                    //         for coord in &v1.1 {
                    //             b.board_arr[map2d_coord(&coord)] = Piece::None;
                    //         }
                    //         b.board_arr[map2d_coord(&bf)] = Piece::None;
                    //         b.first = None;
                    //     }
                    // }
                }
            } else if board.first.is_none() {
                if board.board_arr[map2d(&x, &y)] == board.turn
                    && !available_moves(board, x, y).is_empty()
                {
                    board.first = Option::from((x, y));
                    //println!("selected first");
                }
            }
        }
    }
}

pub fn view(board: &Board) -> Element<BoardStateMsg> {
    let mut col: Vec<Element<BoardStateMsg>> = Vec::with_capacity(Board::WIDTH);
    for y in 0..Board::WIDTH {
        let mut v: Vec<Element<BoardStateMsg>> = Vec::with_capacity(Board::WIDTH);
        for x in 0..Board::WIDTH {
            v.push(
                Button::new(match board.board_arr[map2d(&(x as i16), &(y as i16))] {
                    Piece::None => iced::widget::image(""),
                    Piece::Red => widget::image("assets/red.png"),
                    Piece::Black => widget::image("assets/black.png"),
                })
                .on_press(BoardStateMsg::Selection(x as i16, y as i16))
                .width(100)
                .height(100)
                .style(get_space_color(board, y as i16, x as i16))
                .into(),
            );
        }
        col.push(widget::Row::from_vec(v).into());
    }
    widget::Column::from_vec(col).into()
}

//(available_moves_coord(b, b.first.unwrap())
//         .contains(&MoveAction::Move((x, y))) || available_moves_coord(b, b.first.unwrap())
//         .contains(&MoveAction::Capture(((x, y), (_, _)))))
//&(x, y)
fn get_space_color(b: &Board, y: i16, x: i16) -> fn(&Theme, Status) -> Style {
    if b.first.is_some() && b.first.unwrap() == (x, y) {
        style_selected
    } else if b.first.is_some()
        && contains_coords(&available_moves_coord(b, b.first.unwrap()), b, (x, y))
    {
        style_available
    } else if (x + y) % 2 == 0 {
        style_white
    } else {
        style_black
    }
}

//todo random colors
fn style_white(_t: &Theme, _s: Status) -> Style {
    Style {
        background: Option::from(Background::Color(match _s {
            Status::Hovered => color!(250, 250, 250),
            _ => color!(216, 183, 159),
        })),
        text_color: Color::default(),
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
        text_color: Color::default(),
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
        text_color: Color::default(),
        border: Border {
            color: color!(100, 100, 100),
            width: 1f32,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
    }
}

fn style_available(_t: &Theme, _s: Status) -> Style {
    Style {
        background: Option::from(Background::Color(match _s {
            Status::Hovered => color!(254, 183, 82),
            _ => color!(254, 226, 82),
        })),
        text_color: Color::default(),
        border: Border {
            color: color!(254, 183, 82),
            width: 1f32,
            radius: Radius::new(0),
        },
        shadow: Shadow::default(),
    }
}
