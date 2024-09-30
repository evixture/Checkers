//use std::collections::LinkedList;
use tauri;
enum Piece {
    None,
    Red,
    Black,
}

//for 8x8 grid
fn two_to_one(&x: &usize, &y: &usize) -> usize {
    (y * 8) + x
}

struct Board {
    board_arr: Vec<Piece>, //2d 8x8 array
                           //there is probably a much more simple representation of a checkers board
                           //since half of the board cannot be used and pieces can only move diagonally
}

impl Board {
    pub fn new() -> Self {
        let mut new_var: Vec<Piece> = Vec::with_capacity(64);
        //new_var.set_len(64 : usize);
        for y in 0..8 {
            for x in 0..8 {
                // if (x % 2 == 0) && (y % 2 == 1) {
                //     if y <= 2 {
                //         //new_var[two_to_one(&x, &y)] = Piece::Black;
                //         new_var.push(Piece::Black);
                //     } else if y >= 5 {
                //         //new_var[two_to_one(&x, &y)] = Piece::Red;
                //         new_var.push(Piece::Red);
                //     }
                // } else if (x % 2 == 1) && (y % 2 == 0) {
                //     if y <= 2 {
                //         //new_var[two_to_one(&x, &y)] = Piece::Black;
                //         new_var.push(Piece::Black);
                //     } else if y >= 5 {
                //         //new_var[two_to_one(&x, &y)] = Piece::Red;
                //         new_var.push(Piece::Red);
                //     }
                // } else {
                //     //new_var[two_to_one(&x, &y)] = Piece::None;
                //     new_var.push(Piece::None);
                // }

                if (y == 0 || y == 2) && (x % 2 == 0) {
                    new_var.push(Piece::Black);
                } else if (y == 5 || y == 7) && (x % 2 == 1) {
                    new_var.push(Piece::Red);
                } else if (y == 1) && (x % 2 != 0) {
                    //problem >BBBBXXXX
                    new_var.push(Piece::Black);
                } else if (y == 6) && (x % 2 == 0) {
                    new_var.push(Piece::Red);
                } else {
                    new_var.push(Piece::None);
                }
            }
        }
        println!("size of board: {}", new_var.len());
        Board { board_arr: new_var }
    }

    fn print(self) {
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
}

fn main() {
    let b: Board = Board::new();
    b.print();
}
