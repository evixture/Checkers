//use std::collections::LinkedList;
use tauri;
enum Piece {
    None,
    Red,
    Black,
}

fn two_to_one(&x: &i32, &y: &i32) -> i32 {
    (y * x) + x
}

struct Board {
    board_arr: [Piece; 64], //2d 8x8 array
                            //there is probably a much more simple representation of a checkers board
                            //since half of the board cannot be used and pieces can only move diagonally
}

impl Board {
    pub fn new(self) -> Self {
        for y in 0..8 {
            for x in 0..8 {
                if (x % 2 == 0) && (y % 2 == 1) {
                    if y <= 2 {
                        self.board_arr[two_to_one(&x, &y)] = Piece::Black;
                    } else if y >= 5 {
                        self.board_arr[two_to_one(&x, &y)] = Piece::Red;
                    }
                } else if (x % 2 == 1) && (y % 2 == 0) {
                    if y <= 2 {
                        self.board_arr[two_to_one(&x, &y)] = Piece::Black;
                    } else if y[x] >= 5 {
                        self.board_arr[two_to_one(&x, &y)] = Piece::Red;
                    }
                } else {
                    self.board_arr[two_to_one(&x, &y)] = Piece::None;
                }
            }
        }
        return self;
    }

    pub fn print(self) {
        for y in 0..8 {
            for x in 0..8 {
                print!(stringify!(&x));
            }
            println!();
        }
    }
}

fn main() {
    let b = Board::new();
    b.print();
}
