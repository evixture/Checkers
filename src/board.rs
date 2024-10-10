use iced::widget::Column;
use iced::widget::Row;
use iced::widget::{button, Container};
use iced::Element;

#[derive(Clone)]
enum Piece {
    None,
    Red,
    Black,
}

#[derive(Clone, Debug)]
pub enum MoveMessage {
    Selection,
    None,
}

//for 8x8 grid
fn map2d(x: &usize, y: &usize, width: &usize) -> usize {
    (y * width) + x
}

//#[derive(Clone)]
pub struct Board {
    board_arr: Vec<Piece>,
    //board_view: Vec<Element<MoveMessage>>, //Column<'a, MoveMessage>,
}

//manually implement default instead of #derive'ing
impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    const WIDTH: usize = 8;

    pub fn new() -> Self {
        let mut new_board_arr: Vec<Piece> = Vec::with_capacity(Self::WIDTH);
        let mut new_board_vec: Vec<Element<MoveMessage>> = Vec::with_capacity(Self::WIDTH);

        for y in 0..Self::WIDTH {
            let mut row: Vec<Element<MoveMessage>> = Vec::with_capacity(Self::WIDTH);
            for x in 0..Self::WIDTH {
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
                row.push(button("something").into());
            }
            new_board_vec.push(Row::from_vec(row).into());
        }
        Board {
            board_arr: new_board_arr,
            //board_view: new_board_vec, //Column::from_vec(new_board_vec),
        }
    }

    pub fn print(self) {
        for y in 0..Self::WIDTH {
            for x in 0..Self::WIDTH {
                let mut str: String = String::new();
                match self.board_arr[map2d(&x, &y, &Self::WIDTH)] {
                    Piece::None => str.push_str("."),
                    Piece::Black => str.push_str("B"),
                    Piece::Red => str.push_str("R"),
                }
                print!("{}", str);
            }
            println!();
        }
    }

    pub fn update(&self) {}

    pub fn get_view(&self) -> Element<MoveMessage> {
        //Column::from_vec(self.board_view).into()
        //button("something").into()
        //let ret = iced::widget::column!();

        // for y in 0..Self::WIDTH {
        //     let mut row : Row<Element<MoveMessage>> = Row::new();
        //     for x in 0..Self::WIDTH {
        //         row.push(button("X"));
        //     }
        //     ret.push(row);
        // }
        // ret.into()
        let mut r: Row<iced::widget::Button<MoveMessage>> = iced::widget::Row::new();
        for y in 0..Self::WIDTH {
            r.push(button("x"));
        }

        iced::widget::column!(button("1"), button("2")).into()
    }
}
