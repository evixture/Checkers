use iced::widget::button;
use iced::widget::Row;
use iced::Element;

#[derive(Clone, Eq, PartialEq)]
pub enum Piece {
    None,
    Red,
    Black,
}

#[derive(Clone, Debug)]
pub enum BoardStateMsg {
    Selection(usize, usize),
}

pub fn piece_to_string(piece: &Piece) -> String {
    match piece {
        Piece::None => "X".to_string(),
        Piece::Red => "R".to_string(),
        Piece::Black => "B".to_string(),
    }
}

pub fn map2d(x: &usize, y: &usize, width: &usize) -> usize {
    (y * width) + x
}

pub fn map2d_coord(coord: &(usize, usize), width: &usize) -> usize {
    (coord.1 * width) + coord.0
}

pub fn available_moves(b: &Board, sx: usize, sy: usize) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = vec![];
    // -1, -1           1, -1
    //          sx, sy
    // -1, 1            1, 1

    //nw
    if b.turn == Piece::Red
        && sx >= 1
        && sy >= 1
        && b.board_arr[map2d(&(sx - 1), &(sy - 1), &Board::WIDTH)] == Piece::None
    {
        ret.push((&sx - 1, &sy - 1));
    }
    //ne
    if b.turn == Piece::Red
        && sx <= Board::WIDTH - 2
        && sy >= 1
        && b.board_arr[map2d(&(sx + 1), &(sy - 1), &Board::WIDTH)] == Piece::None
    {
        ret.push((&sx + 1, &sy - 1));
    }
    //sw
    if b.turn == Piece::Black
        && sx >= 1
        && sy <= Board::WIDTH - 2
        && b.board_arr[map2d(&(sx - 1), &(sy + 1), &Board::WIDTH)] == Piece::None
    {
        ret.push((&sx - 1, &sy + 1));
    }
    //se
    if b.turn == Piece::Black
        && sx <= Board::WIDTH - 2
        && sy <= Board::WIDTH - 2
        && b.board_arr[map2d(&(sx + 1), &(sy + 1), &Board::WIDTH)] == Piece::None
    {
        ret.push((&sx + 1, &sy + 1));
    }
    ret
}

pub fn available_moves_coord(
    b: &crate::board::Board,
    s_coords: (usize, usize),
) -> Vec<(usize, usize)> {
    available_moves(b, s_coords.0, s_coords.1)
}

pub struct Board {
    pub board_arr: Vec<Piece>,
    pub first: Option<(usize, usize)>,
    pub second: Option<(usize, usize)>,
    pub turn: Piece,
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
            first: None,
            second: None,
            turn: Piece::Black,
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
