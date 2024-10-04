pub mod board;
pub mod window;
//use iced::widget::{button, column, text, Column};
//use iced::Center;

use board::Board;
use window::CheckersWindow;
fn main() -> iced::Result {
    let b: Board = Board::new();
    b.print();

    iced::run(
        "Hello, world!",
        CheckersWindow::update,
        CheckersWindow::view,
    )
}
