use iced::widget::{column, container, text, text_editor};
use iced::{Element, Theme};

fn main() -> iced::Result {
    // iced::run(Editor::update, Editor::view)
    iced::application(Editor::new, Editor::update, Editor::view)
        .theme(Editor::theme)
        .title(Editor::title)
        .window_size((800, 500))
        .run()
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
