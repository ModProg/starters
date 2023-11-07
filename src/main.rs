use std::process::{exit, Command};

use clap::Parser;
use executable_finder::{executables, Executable};

use gooey::context::AsEventContext;
use gooey::kludgine::app::winit::keyboard::KeyCode;
use gooey::kludgine::figures::IntoSigned;
use gooey::widget::{MakeWidget, ManagedWidget, Widget, WidgetInstance, WidgetRef, IGNORED};
use gooey::widgets::{Button, Expand, Input, Scroll, Stack};
use gooey::{kludgine::app::winit::window::WindowLevel, Run};
use gooey::{value::Dynamic, widget::Children};

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

impl Data {
    fn build(&self, filter: &str) -> Children {
        match self {
            Data::Commands(this) => this
                .iter()
                .filter(|this| this.name.to_lowercase().contains(&filter.to_lowercase()))
                .take(20)
                .map(|it| {
                    let path = it.path.clone();
                    Button::new(it.name.to_owned()).on_click(move |_| {
                        _ = Command::new(&path).spawn();
                        exit(0);
                    })
                })
                .collect(),
            Data::None => Children::default(),
        }
    }
}

pub fn main() {
    let results: Data = match Cmd::parse() {
        Cmd::Commands => Data::Commands({
            let mut data = executables().unwrap();
            data.sort();
            data.dedup();
            eprintln!("{}", data.len());
            data
        }),
        Cmd::Windows => todo!(),
        Cmd::Apps => todo!(),
    };

    let filter = Dynamic::new("".to_string());
    let programs = filter.map_each(move |filter: &String| results.build(filter));

    let input = Input::new(filter)
        .on_key(|key| match key.physical_key {
            KeyCode::Escape => exit(0),
            _ => IGNORED,
        })
        .make_widget();
    let mut window = gooey::window::Window::for_widget(Root(
        WidgetRef::new(Expand::new(Stack::rows(
            Children::new()
                .with_widget(input.clone())
                .with_widget(Scroll::vertical(Stack::rows(programs))),
        ))),
        input,
    ));
    window.attributes.window_level = WindowLevel::AlwaysOnTop;
    window.attributes.resizable = false;
    window.attributes.title = "Hello World".to_owned();
    window.run().unwrap();
    // },
}

#[derive(Debug)]
struct Root(WidgetRef, WidgetInstance);

impl Widget for Root {
    fn redraw(&mut self, context: &mut gooey::context::GraphicsContext<'_, '_, '_, '_, '_>) {
        let widget = self.0.mounted(&mut context.as_event_context());
        context.for_other(&widget).redraw()
    }

    fn layout(
        &mut self,
        available_space: gooey::kludgine::figures::Size<gooey::ConstraintLimit>,
        context: &mut gooey::context::LayoutContext<'_, '_, '_, '_, '_>,
    ) -> gooey::kludgine::figures::Size<gooey::kludgine::figures::units::UPx> {
        let widget = self.0.mounted(&mut context.as_event_context());
        let size = context.for_other(&widget).layout(available_space);
        context.set_child_layout(&widget, size.into_signed().into());
        size
    }

    // fn blur(&mut self, _context: &mut gooey::context::EventContext<'_, '_>) {
    //     exit(0)
    // }

    fn accept_focus(&mut self, context: &mut gooey::context::EventContext<'_, '_>) -> bool {
        true
    }

    fn hit_test(
        &mut self,
        location: gooey::kludgine::figures::Point<gooey::kludgine::figures::units::Px>,
        context: &mut gooey::context::EventContext<'_, '_>,
    ) -> bool {
        true
    }

    fn focus(&mut self, context: &mut gooey::context::EventContext<'_, '_>) {
        context.for_other(&self.1).unwrap().focus()
    }
}
