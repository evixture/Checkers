//use std::collections::LinkedList;
use tauri;
enum Piece {
    None,
    Red,
    Black,
}

struct Board {
    boardArr: [[Piece; 8]; 8], //2d 8x8 array
    //there is probably a much more simple representation of a checkers board
    //since half of the board cannot be used and pieces can only move diagonally
}

impl Board {
    pub fn new(&self) -> Self {

        for y in &self.boardArr.len(){
            for x in &self.boardArr[y].len() {
                if (x % 2 == 0) && (y[x] % 2 == 1) {
                    if y[x] <= 2 {
                        &self.boardArr[y][x] = Piece::Black;
                    } else if y[x] >= 5 {
                        &self.boardArr[y][x] = Piece::Red;
                    }
                } else if (x % 2 == 1) && (y[x] % 2 == 0) {
                    if y[x] <= 2 {
                        &self.boardArr[y][x] = Piece::Black;
                    } else if y[x] >= 5 {
                        &self.boardArr[y][x] = Piece::Red;
                    }
                } else {
                    &self.boardArr[y][x] = Piece::None;
                }
            }
        }
        return &self;
    }

    pub fn print(self) {
        for y in self.boardArr.len() {
            for x in self.boardArr[y].len() {
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
