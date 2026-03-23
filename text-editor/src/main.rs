use iced::Element;
use iced::widget::text;

fn main() -> iced::Result {
    iced::run(Editor::update, Editor::view)
}

#[derive(Default)]
struct Editor;

#[derive(Debug, Clone)]
enum Message {}

impl Editor {
    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Text Editor")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, iced!").into()
    }
}
