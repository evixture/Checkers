use crate::window::Message;
use iced::widget::{button, column, text, Column, Row};
use iced::Element;

enum Piece {
    None,
    Red,
    Black,
}

enum MoveMessage {
    Selection(i32, i32),
}

//for 8x8 grid
fn two_to_one(&x: &usize, &y: &usize) -> usize {
    (y * 8) + x
}

pub struct Board {
    board_arr: Vec<Piece>,
    board_view: iced::widget::Column<MoveMessage>,
}

impl Board {
    pub fn new() -> Self {
        let mut new_board_arr: Vec<Piece> = Vec::with_capacity(64);
        let mut new_board_view = iced::widget::Column::new();

        //new_var.set_len(64 : usize);
        for y in 0..8 {
            //let mut row: iced::widget::Row<MoveMessage> = iced::widget::Row::new();
            let mut row: Vec<Element<MoveMessage>> = Vec::with_capacity(8);
            for x in 0..8 {
                if (y == 0 || y == 2) && (x % 2 == 0) {
                    new_board_arr.push(Piece::Black);
                } else if (y == 5 || y == 7) && (x % 2 == 1) {
                    new_board_arr.push(Piece::Red);
                } else if (y == 1) && (x % 2 != 0) {
                    new_board_arr.push(Piece::Black);
                } else if (y == 6) && (x % 2 == 0) {
                    new_board_arr.push(Piece::Red);
                } else {
                    new_board_arr.push(Piece::None);
                }
                row.push(iced::widget::Button("something"));
                //Row::push(row, iced::widget::Button("something")); //.push(iced::widget::Button("something"));
                //row.push(iced::widget::Button("something"));
            }
            new_board_view.push(iced::widget::Row::from_vec(row));
        }
        //println!("size of board: {}", new_var.len());
        Board {
            board_arr: new_board_arr,
            board_view: new_board_view,
        }
    }

    pub fn print(self) {
        for y in 0..8 {
            for x in 0..8 {
                let mut str: String = String::new();
                match self.board_arr[two_to_one(&x, &y)] {
                    Piece::None => str.push_str("."),
                    Piece::Black => str.push_str("B"),
                    Piece::Red => str.push_str("R"),
                }
                print!("{}", str);
            }
            println!();
        }
    }

    pub fn get_view() -> iced::Element<'_, MoveMessage> {}
}
