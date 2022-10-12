use clap::Parser;
use executable_finder::{executables, Executable};
use iced::keyboard::{self, KeyCode};
use iced::widget::{button, column, scrollable, text, text_input};
use iced::{executor, theme, window, Application, Command, Element, Event, Length, Theme};
use iced_native::window::Event as WindowEvent;

#[derive(Debug, Parser)]
enum Cmd {
    Commands,
    Windows,
    Apps,
}

#[derive(Clone)]
enum Data {
    Commands(Vec<Executable>),
    None,
}

impl Default for Data {
    fn default() -> Self {
        Data::None
    }
}

impl Data {
    fn build(self, filter: &str) -> Vec<Element<Message>> {
        match self {
            Data::Commands(this) => this
                .into_iter()
                .filter(|this| this.name.to_lowercase().contains(&filter.to_lowercase()))
                .map(|this| {
                    button(text(this.name))
                        .width(Length::Fill)
                        .style(theme::Button::Text)
                        .into()
                })
                .collect(),
            Data::None => vec![],
        }
    }
}

pub fn main() -> iced::Result {
    let results: Data = match Cmd::parse() {
        Cmd::Commands => Data::Commands({
            let mut data = executables().unwrap();
            data.sort();
            data.dedup();
            data
        }),
        Cmd::Windows => todo!(),
        Cmd::Apps => todo!(),
    };
    Starter::run(iced::Settings {
        window: window::Settings {
            always_on_top: true,
            resizable: false,
            size: (500, 200),
            decorations: false,
            ..Default::default()
        },
        flags: results,
        ..Default::default()
    })
}

#[derive(Default)]
struct Starter {
    should_exit: bool,
    was_focused: bool,
    data: Data,
    filter: String,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    Filter(String),
}

impl Application for Starter {
    type Executor = executor::Default;
    type Message = Message;

    type Theme = Theme;

    type Flags = Data;

    fn new(data: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                data,
                ..Self::default()
            },
            text_input::focus(text_input::Id::new("filter")),
        )
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
        // match dark_light::detect() {
        //     dark_light::Mode::Dark => Theme::Dark,
        //     dark_light::Mode::Light => Theme::Light,
        // }
    }

    fn title(&self) -> String {
        String::from("Starters")
    }

    fn view(&self) -> Element<Message> {
        // TODO Custom scrollable for performance
        let r = column![
            text_input("Filter", &self.filter, |s| { Message::Filter(s) })
                .id(text_input::Id::new("filter"))
                .padding(5),
            scrollable(column(self.data.clone().build(&self.filter)))
        ]
        .padding(5)
        .into();
        r
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EventOccurred(Event::Window(WindowEvent::Focused)) => self.was_focused = true,
            Message::EventOccurred(Event::Window(WindowEvent::Unfocused)) if self.was_focused => {
                self.should_exit = true
            }
            Message::EventOccurred(Event::Keyboard(
                keyboard::Event::KeyPressed {
                    key_code: KeyCode::Escape,
                    ..
                }
                | keyboard::Event::KeyReleased {
                    key_code: KeyCode::Escape,
                    ..
                },
            )) => self.should_exit = true,
            Message::EventOccurred(Event::Keyboard(keyboard::Event::CharacterReceived('c'))) => {
                // self.filter.push(c)
            }
            Message::Filter(filter) => self.filter = filter,
            _ => (),
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }
}
