use iced::widget::button;
use iced::widget::Row;
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

#[derive(Clone, Debug, Default)]
pub enum Turn {
    #[default]
    Start,
    Red,
    Black,
    End,
}

#[derive(Clone, Debug)]
pub enum BoardStateMsg {
    Turn(Turn),
    Selection(u8, u8),
    None,
}

//for 8x8 grid
pub fn map2d(x: &usize, y: &usize, width: &usize) -> usize {
    (y * width) + x
}

pub struct Board {
    pub board_arr: Vec<Piece>,
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
        let mut new_board_vec: Vec<Element<BoardStateMsg>> = Vec::with_capacity(Self::WIDTH);

        for y in 0..Self::WIDTH {
            let mut row: Vec<Element<BoardStateMsg>> = Vec::with_capacity(Self::WIDTH);
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
}
