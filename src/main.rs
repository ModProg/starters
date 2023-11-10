use std::process::{exit, Command};

use clap::Parser;
use executable_finder::{executables, Executable};
use gooey::context::AsEventContext;
use gooey::kludgine::app::winit::keyboard::KeyCode;
use gooey::kludgine::figures::units::Px;
use gooey::kludgine::figures::{IntoSigned, Size};
use gooey::styles::DimensionRange;
use gooey::widget::{MakeWidget, Widget, WidgetInstance, WidgetRef, IGNORED};
use gooey::widgets::{Input, Resize, Scroll, Stack};
use gooey::window::Window;
use gooey::{kludgine::app::winit::window::WindowLevel, Run};
use gooey::{value::Dynamic, widget::Children};

mod fuzzy;

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
        let fuzzy = fuzzy::Fuzzy::new();
        match self {
            Data::Commands(commands) => fuzzy.filter(filter, commands),
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
    let mut window = Window::for_widget(Root(WidgetRef::new(Resize::to(
        Size::<DimensionRange>::new(Px(600), ..Px(400)),
        Stack::rows(input.clone().and(Stack::rows(programs))),
    ))));
    window.attributes.window_level = WindowLevel::AlwaysOnTop;
    window.attributes.resizable = false;
    window.attributes.title = "Starters".to_owned();
    window.run().unwrap();
    // },
}

#[derive(Debug)]
struct Root(WidgetRef);

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
}
