#![windows_subsystem = "windows"]

use std::fmt::Error;
use std::path::PathBuf;

use iced::alignment;
use iced::executor;
use iced::theme;
use iced::widget::{checkbox, column, container, Text};
use iced::{Application, Command, Element, Length, Settings, Theme};

fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    src_path: String,
    dest_path: String,
    file_type: String,
    recursive: bool,
    progress: f32,
    organising: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SrcPathChanged(Result<PathBuf, Error>),
    DestPathChanged(Result<PathBuf, Error>),
    FileTypeChanged(String),
    RecursiveChanged(),
    ProgressChanged(),
}

impl Application for Example {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                src_path: String::from(""),
                dest_path: String::from(""),
                file_type: String::from(""),
                recursive: false,
                progress: 0.0,
                organising: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("File Organiser")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SrcPathChanged(res) => {
                self.src_path = String::from("src path changed");
            }
            Message::DestPathChanged(res) => {
                self.dest_path = String::from("dest path changed");
            }
            Message::FileTypeChanged(file_type) => {
                self.file_type = String::from("file type changed");
            }
            Message::RecursiveChanged() => {
                self.recursive = !self.recursive;
            }
            Message::ProgressChanged() => {
                self.progress += 0.1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let heading: Text = Text::new("AGM Price Calculator").size(50);

        let mut contents = column![];

        contents = contents.push(heading);

        container(contents.spacing(20).padding(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center)
            .into()
    }
}
