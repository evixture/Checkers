use iced::widget::button;
use iced::widget::Column;
use iced::widget::Row;
use iced::Element;

#[derive(Default)]
enum Piece {
    #[default]
    None,
    Red,
    Black,
}

#[derive(Clone, Debug, Default)]
pub enum MoveMessage {
    #[default]
    Selection,
    None,
}

//for 8x8 grid
fn map2d(x: &usize, y: &usize, width: &usize) -> usize {
    (y * width) + x
}

//#[derive(Default)]
pub struct Board<'a> {
    board_arr: Vec<Piece>,
    board_view: Vec<Element<'a, MoveMessage>>, //Column<'a, MoveMessage>,
}

impl Default for Board<'_> {
    fn default() -> Self {
        Board::new()
    }
}

impl<'a> Board<'a> {
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
            board_view: new_board_vec, //Column::from_vec(new_board_vec),
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

        // fn default() -> Self {
        //     Board::new()
        // }
    }

    pub fn update(&self) {}

    pub fn get_view(self) -> Element<'a, MoveMessage> {
        //self.board_view.into()
        Column::from_vec(self.board_view).into()
    }
}

// impl<'a> Default for Column<'a, MoveMessage> {
//     fn default() -> Self {
//         Column::new()
//     }
// }
