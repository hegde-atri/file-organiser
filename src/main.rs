use iced::{
    theme,
    widget::{button, horizontal_space, row},
    Color, Element, Sandbox, Settings,
};
mod gui;
mod lib;

fn main() -> iced::Result {
    FileOrganiser::run(Settings::default())
}

pub struct FileOrganiser {
    steps: Steps,
}

impl Sandbox for FileOrganiser {
    type Message = Message;

    fn new() -> FileOrganiser {
        FileOrganiser {
            steps: Steps::new(),
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", self.steps.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let FileOrganiser { steps } = self;
        let mut controls = row![];

        if steps.has_previous() {
            controls = controls.push(
                button("Back")
                    .on_press(Message::BackPressed)
                    .style(theme::Button::Secondary),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button("Next")
                    .on_press(Message::NextPressed)
                    .style(theme::Button::Primary),
            );
        }

        // FIXME
        // let content: Element<_> =
        // column![steps.view(self.debug).map(Message::StepMessage), controls,]
        //     .max_width(540)
        //     .spacing(20)
        //     .padding(20)
        //     .into();
    }
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    // FIXME
    fn new() -> Steps {}
}

enum Step {
    Welcome,
    ChooseDirs,
    ChooseOptions,
    Progress,
    End,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    SliderChanged(u8),
    SpacingChanged(u16),
    TextSizeChanged(u16),
    TextColorChanged(Color),
    ImageWidthChanged(u16),
    InputChanged(String),
    TogglerChanged(bool),
}
