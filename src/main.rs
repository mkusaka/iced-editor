use iced::{Element, Sandbox, Settings};
use iced::widget::text;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor;

#[derive(Debug)]
enum Message {}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text("hello, iced!").into()
    }
}