use iced::widget::{button, column, text, Column};
use iced::Center;

#[derive(Default)]
pub struct CheckersWindow {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl CheckersWindow {
    //iced::run expects a state and message from update function
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    //iced::run expects at least a state from view function
    pub fn view(&self) -> iced::Element<'_, Message> {
        // iced::widget::column![
        //     button("Increment").on_press(Message::Increment),
        //     text(self.value).size(50),
        //     button("Decrement").on_press(Message::Decrement)
        // ]
        // .padding(20)
        // .align_x(Center)

        iced::widget::container(iced::widget::row![button("Red")]).into()
    }
}
