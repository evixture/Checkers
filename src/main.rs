use iced::Size;

pub mod board;
pub mod window;

//todo try to reduce clone calls
fn main() -> iced::Result {
    iced::application("Checkers", window::update, window::view)
        .window_size(Size {
            width: 800f32,
            height: 800f32,
        })
        .resizable(false)
        .run()
}
