#![windows_subsystem = "windows"]

use iced::pure::widget::{Button, Column, Container, Row, Text};
use iced::pure::Sandbox;
use iced::{window, Settings};

struct Counter {
    count: i32,
}

#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = CounterMessage;
    fn new() -> Self {
        Counter { count: 0 }
    }
    fn title(&self) -> String {
        String::from("Counter App")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count -= 1,
        }
    }
    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        let app_name = Text::new("Authenticator");
        let btn_add = Button::new("Hello");
        let header = Row::new().push(app_name).push(btn_add);

        let label = Text::new(format!("Count: {}", self.count));
        let incr = Button::new("Increment").on_press(CounterMessage::Increment);
        let decr = Button::new("Decrement").on_press(CounterMessage::Decrement);
        let col = Column::new().push(header).push(incr).push(label).push(decr);
        Container::new(col)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Top)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: window::Settings {
            size: (400, 600),
            resizable: (false),
            decorations: (true),
            ..Default::default()
        },
        ..Default::default()
    };
    Counter::run(settings)
}
