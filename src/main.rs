//! Elbey - a desktop app launcher
use iced::widget::{Column, button, column, text};
use iced::{Font, Pixels, Task, Theme};
use iced_layershell::{to_layer_message, Application};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use lazy_static::lazy_static;

lazy_static! {
    static ref PROGRAM_NAME: String = String::from("Aratrat");
}

fn main() -> Result<(), iced_layershell::Error> {
    let iced_settings = Settings {
        layer_settings: LayerShellSettings {
            size: Some((16, 0)), // 8 pixels wide, 0 for height means fill available
            exclusive_zone: 16, // Optional: set an exclusive zone for the 8px width
            anchor: Anchor::Left,
            start_mode: StartMode::Active,
            layer: Layer::Overlay,
            margin: (0, 0, 0, 0),
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            events_transparent: false,
        },
        flags: Flags {},
        id: Some(PROGRAM_NAME.to_string()),
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels::from(18),
        antialiasing: true,
        virtual_keyboard_support: None,
    };

    Ararat::run(iced_settings)
}

pub struct Ararat {
    state: Counter,
    flags: Flags,
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone)]
pub struct Flags {}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

impl Application for Ararat {
    type Message = Message;
    type Flags = Flags;
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(flags: Self::Flags) -> (Self, iced::Task<Self::Message>) {
        (Self { state: Counter::default(), flags }, Task::none() )
    }

    fn namespace(&self) -> String {
        PROGRAM_NAME.to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Task<Self::Message> {
        match message {
            Message::Increment => {
                self.state.value += 1;
            }
            Message::Decrement => {
                self.state.value -= 1;
            }
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.state.value).size(50).font(Font::MONOSPACE),
            button("Decrement").on_press(Message::Decrement)
        ]
        .align_x(iced::Alignment::Start)
        .into()
    }
}
