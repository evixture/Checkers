use iced::widget::Column;
use iced::widget::Row;
use iced::widget::{button, Container};
use iced::Element;

#[derive(Clone)]
pub enum Piece {
    None,
    Red,
    Black,
}

pub fn piece_to_string(piece: &Piece) -> String {
    match piece {
        Piece::None => "X".to_string(),
        Piece::Red => "R".to_string(),
        Piece::Black => "B".to_string(),
    }
}

#[derive(Clone, Debug)]
pub enum MoveMessage {
    Selection,
    None,
}

//for 8x8 grid
pub fn map2d(x: &usize, y: &usize, width: &usize) -> usize {
    (y * width) + x
}

//#[derive(Clone)]
pub struct Board {
    pub board_arr: Vec<Piece>,
    //board_view: Vec<Element<MoveMessage>>, //Column<'a, MoveMessage>,
}

//manually implement default instead of #derive'ing
impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    pub const WIDTH: usize = 8;

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

    // pub fn print(self) {
    //     for y in 0..Self::WIDTH {
    //         for x in 0..Self::WIDTH {
    //             let mut str: String = String::new();
    //             match self.board_arr[map2d(&x, &y, &Self::WIDTH)] {
    //                 Piece::None => str.push_str("."),
    //                 Piece::Black => str.push_str("B"),
    //                 Piece::Red => str.push_str("R"),
    //             }
    //             print!("{}", str);
    //         }
    //         println!();
    //     }
    // }

    // pub fn update(&self) {}

    //     pub fn get_view(&self) -> Element<MoveMessage> {
    //         let mut v: Vec<iced::Element<MoveMessage>> = Vec::with_capacity(Self::WIDTH);
    //         for x in 0..Self::WIDTH {
    //             let str: String;
    //             match self.board_arr[x] {
    //                 Piece::None => str = "X".to_string(),
    //                 Piece::Red => str = "R".to_string(),
    //                 Piece::Black => str = "B".to_string(),
    //             };
    //
    //             v.push(button(str.as_str()).on_press(MoveMessage::Selection).into());
    //         }
    //         iced::widget::Row::from_vec(v).into()
    //     }
}
