use crate::board::*;
use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::Button;
use iced::{color, widget, Background, Border, Color, Element, Shadow, Theme};

pub fn update(b: &mut Board, msg: BoardStateMsg) {
    match msg {
        BoardStateMsg::Selection(x, y) => {
            if b.first.is_some() {
                //deselect by selecting same spot
                let bf = b.first.unwrap();
                if b.first.unwrap() == (x, y) {
                    b.first = None;
                    return;
                    //println!("deselected first");
                } else {
                    //println!("selected second");
                    let v1 = available_moves_coord(b, bf);
                    for av_move in v1.0 {
                        //println!("{:?} {}, {}", av_move, x, y);
                        if av_move == (x, y) {
                            //println!("matched move");
                            match b.turn {
                                Piece::Black => {
                                    b.turn = Piece::Red;
                                    b.board_arr[map2d_coord(&av_move)] = Piece::Black;
                                    println!("move black");
                                    if !v1.1.is_empty() {
                                        println!("{}, {}", v1.1[0].0, v1.1[0].1);
                                    }
                                }
                                Piece::Red => {
                                    b.turn = Piece::Black;
                                    b.board_arr[map2d_coord(&av_move)] = Piece::Red;
                                    println!("move red");
                                }
                                _ => (),
                            }
                            b.board_arr[map2d_coord(&bf)] = Piece::None;
                            b.first = None;
                        }
                    }
                }
            } else if b.first.is_none() {
                if b.board_arr[map2d(&x, &y)] == b.turn && !available_moves(b, x, y).0.is_empty() {
                    b.first = Option::from((x, y));
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
                Button::new(match board.board_arr[map2d(&x, &y)] {
                    Piece::None => iced::widget::image(""),
                    Piece::Red => widget::image("assets/red.png"),
                    Piece::Black => widget::image("assets/black.png"),
                })
                .on_press(BoardStateMsg::Selection(x, y))
                .width(100)
                .height(100)
                .style(get_space_color(board, y, x))
                .into(),
            );
        }
        col.push(widget::Row::from_vec(v).into());
    }
    widget::Column::from_vec(col).into()
}

fn get_space_color(b: &Board, y: usize, x: usize) -> fn(&Theme, Status) -> Style {
    if b.first.is_some() && b.first.unwrap() == (x, y) {
        style_selected
    } else if b.first.is_some()
        && available_moves_coord(b, b.first.unwrap())
            .0
            .contains(&(x, y))
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
