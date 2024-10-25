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
    Selection(i16, i16),
}

pub fn piece_to_string(piece: &Piece) -> String {
    match piece {
        Piece::None => "X".to_string(),
        Piece::Red => "R".to_string(),
        Piece::Black => "B".to_string(),
    }
}

pub fn piece_opposite(piece: &Piece) -> Piece {
    match piece {
        Piece::Black => Piece::Red,
        Piece::Red => Piece::Black,
        Piece::None => Piece::None,
    }
}

pub fn map2d(x: &i16, y: &i16) -> usize {
    ((y * Board::WIDTH as i16) + x) as usize
}
pub fn map2d_coord(coord: &(i16, i16)) -> usize {
    ((coord.1 * Board::WIDTH as i16) + coord.0) as usize
}

pub fn coord_is_in_bounds(coord: &(i16, i16)) -> bool {
    coord.0 >= 0 && coord.0 < Board::WIDTH as i16 && coord.1 >= 0 && coord.1 < Board::WIDTH as i16
}

//target, capture, origin
type CaptureType = ((i16, i16), (i16, i16), (i16, i16));
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MoveAction {
    Move((i16, i16)),
    Capture(CaptureType),
}

type MoveReturn = Vec<MoveAction>;

pub fn contains_coords(mr: &MoveReturn, c: (i16, i16)) -> bool {
    let mut ret: bool = false;
    for ma in mr {
        //available_moves_coord(b, b.first.unwrap()
        match ma {
            MoveAction::Move((a, b)) => {
                if (&c.0, &c.1) == (a, b) {
                    ret = true
                }
            }
            MoveAction::Capture(((a, b), (_, _), (_, _))) => {
                if (&c.0, &c.1) == (a, b) {
                    ret = true
                }
            }
        }
    }
    ret
}

fn has_captures(mr: MoveReturn) -> bool {
    let mut ret: bool = false;
    for ma in mr {
        match ma {
            MoveAction::Move(_) => (),
            MoveAction::Capture(_) => ret = true,
        }
    }
    ret
}

fn get_captures_rec(
    mr: MoveReturn,
    end: &(i16, i16),
    start: &(i16, i16),
    mut ret: Vec<(i16, i16)>,
) -> Vec<(i16, i16)> {
    //find move
    for ma in &mr {
        match ma {
            MoveAction::Capture(((a, b), (c, d), (e, f))) => {
                if (a, b) == (&end.0, &end.1) {
                    ret.push((*c, *d));
                    ret.append(&mut get_captures_rec(
                        mr.clone(),
                        &(*e, *f),
                        start,
                        ret.clone(),
                    ));
                }
            }
            _ => (),
        }
    }
    ret
}

pub fn get_captures(mr: &MoveReturn, end: &(i16, i16), start: &(i16, i16)) -> Vec<(i16, i16)> {
    let mut ret: Vec<(i16, i16)> = Vec::new();
    let mut cap_end: Option<&MoveAction> = Option::None;

    //find if there is a capture with end coords
    for ma in &mr {
        match ma {
            MoveAction::Capture(((a, b), (_, _), (_, _))) => {
                if (a, b) == (end.0, end.1) {
                    cap_end = Option::from(ma);
                }
            }
            _ => (),
        }
    }

    //do recursive search to find all captured pieces
    if cap_end.is_none() {
        ret = get_captures_rec(mr.clone(), end, start, ret.clone());
    }

    ret
}

//todo move potential move adds to body of function, only needs to be done once
fn av_moves_rec(b: &Board, sx: &i16, sy: &i16, mr: MoveReturn) -> MoveReturn {
    let mut pm_list: Vec<(i16, i16)> = vec![];
    let mut ret: MoveReturn = mr.clone();

    match b.turn {
        Piece::Black => {
            if coord_is_in_bounds(&(sx - 1, sy + 1)) {
                pm_list.push((sx - 1, sy + 1));
            }
            if coord_is_in_bounds(&(sx + 1, sy + 1)) {
                pm_list.push((sx + 1, sy + 1));
            }
        }
        Piece::Red => {
            if coord_is_in_bounds(&(sx - 1, sy - 1)) {
                pm_list.push((sx - 1, sy - 1));
            }
            if coord_is_in_bounds(&(sx + 1, sy - 1)) {
                pm_list.push((sx + 1, sy - 1));
            }
        }
        _ => (),
    }

    for target in pm_list {
        //check capture list to prevent move if jump captures
        if b.board_arr[map2d_coord(&target)] == Piece::None {
            //move
            ret.push(MoveAction::Move(target.clone()));
        } else if b.board_arr[map2d_coord(&target)] != b.turn {
            let new_target = ((target.0 - sx) + target.0, (target.1 - sy) + target.1);
            if coord_is_in_bounds(&new_target) {
                if b.board_arr[map2d_coord(&new_target)] == Piece::None {
                    //ret.0.push(new_target.clone());
                    //ret.1.push(target.clone());
                    ret.push(MoveAction::Capture((
                        new_target.clone(),
                        target.clone(),
                        (*sx, *sy),
                    )));
                    //let mut t: MoveReturn =
                    //    av_moves_rec(b, &new_target.0, &new_target.1, ret.clone());
                    //ret.0.append(&mut t.0);
                    //ret.1.append(&mut t.1);
                    ret.append(&mut av_moves_rec(
                        b,
                        &new_target.0,
                        &new_target.1,
                        ret.clone(),
                    ));
                }
            }
        }
    }
    ret
}

pub fn available_moves(b: &Board, sx: i16, sy: i16) -> MoveReturn {
    let v: MoveReturn = vec![];
    let mut ret: MoveReturn = av_moves_rec(b, &sx, &sy, v);
    // ret.0.sort();
    // ret.1.sort();
    // ret.0.dedup();
    // ret.1.dedup();
    ret.sort();
    ret.dedup();
    ret
}

pub fn available_moves_coord(b: &Board, s_coords: &(i16, i16)) -> MoveReturn {
    available_moves(b, s_coords.0, s_coords.1)
}

pub struct Board {
    pub board_arr: Vec<Piece>,
    pub first: Option<(i16, i16)>,
    pub turn: Piece,
    pub av_moves: Vec<(i16, i16)>,
    pub game_over: bool,
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
            game_over: false,
        }
    }

    pub fn check_game_win(&mut self) {
        let mut has_red = false;
        let mut has_black = false;
        for p in &self.board_arr {
            if p == &Piece::Red {
                has_red = true;
            }
            if p == &Piece::Black {
                has_black = true;
            }
        }
        //win
        if !has_red || !has_black {
            self.game_over = true;
            self.turn = Piece::None;
            for y in 0..Self::WIDTH {
                for x in 0..Self::WIDTH {
                    self.board_arr[map2d(&(x as i16), &(y as i16))] =
                        if !has_red { Piece::Black } else { Piece::Red };
                }
            }
        }
    }

    pub fn print(self) {
        for y in 0..Self::WIDTH {
            for x in 0..Self::WIDTH {
                print!(
                    "{}",
                    piece_to_string(&self.board_arr[map2d(&(x as i16), &(y as i16))])
                );
            }
            println!();
        }
    }
}
