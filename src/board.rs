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

pub fn map2d(x: &usize, y: &usize) -> usize {
    (y * Board::WIDTH) + x
}

pub fn map2d_coord(coord: &(usize, usize)) -> usize {
    (coord.1 * Board::WIDTH) + coord.0
}

type MoveReturn = (Vec<(usize, usize)>, Vec<(usize, usize)>);

//todo simplify, make search coords variables
//todo make vec of 4 move options, for loop thru with match for piece type?
//todo need to plan out best way to detect capture & move
fn av_moves_rec(
    b: &Board,
    sx: &usize,
    sy: &usize,
    mut v: Vec<(usize, usize)>,
    mut w: Vec<(usize, usize)>,
    suspected_capture: bool,
) -> MoveReturn {
    //todo verify bounds
    if sx < &1usize
        || sx >= &(Board::WIDTH - 1usize)
        || sy < &1usize
        || sy >= &(Board::WIDTH - 1usize)
    {
        return (v, w);
    }

    //black
    return if b.turn == Piece::Black {
        //left path
        //normal move and after jump
        if b.board_arr[map2d(&(sx - 1), &(sy + 1))] == Piece::None {
            if suspected_capture {
                w.push((sx + 0, sy + 0));
                println!("capture at {}, {}", sx, sy);
                //suspected_capture = false;
            }
            v.push((sx - 1, sy + 1));
            //possible jump
        } else if b.board_arr[map2d(&(sx - 1), &(sy + 1))] == Piece::Red {
            v.append(&mut av_moves_rec(b, &(sx - 1), &(sy + 1), v.clone(), w.clone(), true).0)
        }
        //right path
        //normal move and after jump
        if b.board_arr[map2d(&(sx + 1), &(sy + 1))] == Piece::None {
            if suspected_capture {
                w.push((sx + 0, sy + 0));
                println!("capture at {}, {}", sx, sy);
                //suspected_capture = false;
            }
            v.push((sx + 1, sy + 1));
            //possible jump
        } else if b.board_arr[map2d(&(sx + 1), &(sy + 1))] == Piece::Red {
            v.append(&mut av_moves_rec(b, &(sx + 1), &(sy + 1), v.clone(), w.clone(), true).0);
        }
        (v, w)
        //red
    } else if b.turn == Piece::Red {
        //left path
        //normal move and after jump
        if b.board_arr[map2d(&(sx - 1), &(sy - 1))] == Piece::None {
            if suspected_capture {
                w.push((sx + 0, sy + 0));
            }
            v.push((sx - 1, sy - 1));
            //possible jump
        } else if b.board_arr[map2d(&(sx - 1), &(sy - 1))] == Piece::Black {
            v.append(&mut av_moves_rec(b, &(sx - 1), &(sy - 1), v.clone(), w.clone(), true).0);
        }
        //right path
        //normal move and after jump
        if b.board_arr[map2d(&(sx + 1), &(sy - 1))] == Piece::None {
            if suspected_capture {
                w.push((sx + 0, sy + 0));
            }
            v.push((sx + 1, sy - 1));
            //possible jump
        } else if b.board_arr[map2d(&(sx + 1), &(sy - 1))] == Piece::Black {
            v.append(&mut av_moves_rec(b, &(sx + 1), &(sy - 1), v.clone(), w.clone(), true).0);
        }
        (v, w)
    } else {
        (v, w)
    };
}

pub fn available_moves(b: &Board, sx: usize, sy: usize) -> MoveReturn {
    let v: MoveReturn = (vec![], vec![]);
    let mut ret: MoveReturn = av_moves_rec(b, &sx, &sy, v.0, v.1, false);
    ret.0.sort();
    ret.1.sort();
    ret.0.dedup();
    ret.1.dedup();
    ret
}

pub fn available_moves_coord(b: &Board, s_coords: (usize, usize)) -> MoveReturn {
    available_moves(b, s_coords.0, s_coords.1)
}

pub struct Board {
    pub board_arr: Vec<Piece>,
    pub first: Option<(usize, usize)>,
    pub turn: Piece,
    pub av_moves: Vec<(usize, usize)>,
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
            av_moves: vec![],
            turn: Piece::Black,
        }
    }

    pub fn print(self) {
        for y in 0..Self::WIDTH {
            for x in 0..Self::WIDTH {
                let mut str: String = String::new();
                match self.board_arr[map2d(&x, &y)] {
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
