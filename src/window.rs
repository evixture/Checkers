use crate::board::{Board, MoveMessage};
use iced::widget::button;
//use iced::{Center, Task};

#[derive(Default)]
pub struct CheckersWindow<'a> {
    board: Board<'a>,
}

impl<'a> CheckersWindow<'a> {
    pub fn new(self) -> Self {
        CheckersWindow {
            board: Board::new(),
        }
    }

    //iced::run expects a state and message from update function
    pub fn update(&mut self, _message: MoveMessage) {
        //message: MoveMessage
        self.board.update();
    }

    //iced::run expects at least a state from view function
    pub fn view(&self) -> iced::Element<MoveMessage>
where
        //'a: 'b, //??? need?
    {
        // iced::widget::column![
        //     button("Increment").on_press(Message::Increment),
        //     text(self.value).size(50),
        //     button("Decrement").on_press(Message::Decrement)
        // ]
        // .padding(20)
        // .align_x(Center)

        //iced::widget::container(iced::widget::row![button("Red")]).into()
        //self.board.get_view()
        button("something").into()
    }
}
