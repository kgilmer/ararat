
use iced::widget::{button, column, text, Column};
use iced::{application, Center, Font, Pixels, Settings};
use lazy_static::lazy_static;

lazy_static! {
    static ref PROGRAM_NAME: String = String::from("Ararat");
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
pub struct Flags {

}

pub fn main() -> iced::Result {
    // iced::run("A cool counter", Counter::update, Counter::view)
    
    let settings: Settings = Settings {
        id: Some(PROGRAM_NAME.to_string()),
        fonts: vec![],
        default_font: Font::MONOSPACE,
        default_text_size: Pixels::from(18),
        antialiasing: true,
    };


    let app = 
    application(PROGRAM_NAME.as_str(), Counter::update, Counter::view).settings(settings);

    app.run()
    // Ararat::run(settings)
}



#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}



impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50).font(Font::MONOSPACE),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
