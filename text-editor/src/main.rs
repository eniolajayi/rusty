use iced::Element;
use iced::widget::{container, text_editor};

fn main() -> iced::Result {
    iced::run(Editor::update, Editor::view)
}

#[derive(Default)]
struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Editor {
    fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Text Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content).on_action(Message::Edit);
        container(input).padding(10).into()
    }
}
