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
        String::from("Heny's Text Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content)
            .on_action(Message::Edit)
            .wrapping(text::Wrapping::Word)
            .max_height(450)
            .min_height(450);
        let cursor = self.content.cursor();

        let position = {
            let cursor_line = cursor.position.line;
            let cursor_column = cursor.position.column;

            text(format!("{}:{}", cursor_line + 1, cursor_column + 1))
        };

        container(column![input, position]).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
