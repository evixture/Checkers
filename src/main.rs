pub mod board;
pub mod window;

//use board::Board;
//use window::CheckersWindow;
fn main() -> iced::Result {
    //Board::new().print();
    iced::run("Hello, world!", window::update, window::view)
    //CheckersWindow::update,
    //CheckersWindow::view,
}
