use iced::widget::{button, column, container, row, space, text, text_editor};
use iced::{Element, Length, Task, Theme};
use std::io;
use std::path::Path;
use std::sync::Arc;

fn main() -> iced::Result {
    // iced::run(Editor::update, Editor::view)
    iced::application(Editor::boot, Editor::update, Editor::view)
        .theme(Editor::theme)
        .title(Editor::title)
        .window_size((800, 500))
        .run()
}

#[derive(Default)]
struct Editor {
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Open,
    FileOpened(Result<Arc<String>, Error>),
}

impl Editor {
    fn boot() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::new(),
                error: None,
            },
            Task::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Heny's Text Editor")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Task::none()
            }
            Message::Open => Task::perform(pick_file(), Message::FileOpened),
            Message::FileOpened(Ok(content)) => {
                self.content = text_editor::Content::with_text(&content);
                Task::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![button("Open").on_press(Message::Open)];

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

        let status_bar = row![space().width(Length::Fill), position];

        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path()).await
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}
