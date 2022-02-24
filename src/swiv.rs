use iced::{
    widget::image::{viewer, Handle, Viewer},
    Container, Element, Length, Sandbox, Settings,
};
use std::process::exit;

pub fn main() -> iced::Result {
    SWIV::run(Settings::default())
}

struct SWIV {
    state: viewer::State,
}

#[derive(Clone, Debug)]
pub enum Message {}

impl Sandbox for SWIV {
    type Message = Message;

    fn new() -> Self {
        SWIV {
            state: viewer::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Simple Wayland Image Viewer")
    }

    fn update(&mut self, message: Message) {
        match message {
            _ => unreachable!(),
        };
    }

    fn view(&mut self) -> Element<Message> {
        let mut args = std::env::args();
        if args.len() != 2 {
            println!("Usage: {} /path/to/image", args.nth(0).unwrap());
            exit(1);
        }
        let path = args.nth(1).unwrap();

        Container::new(
            Viewer::new(&mut self.state, Handle::from_path(path))
                .height(Length::Fill)
                .padding(0)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
