use std::process::exit;

use clap::Parser;
use executable_finder::{executables, Executable};
use gooey::context::AsEventContext;
use gooey::kludgine::app::winit::keyboard::KeyCode;
use gooey::kludgine::figures::units::Px;
use gooey::kludgine::figures::{IntoSigned, Size};
use gooey::styles::{DimensionRange, ThemePair};
use gooey::widget::{MakeWidget, Widget, WidgetRef, IGNORED};
use gooey::widgets::{Expand, Input, Resize, Stack};
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
}

impl Data {
    fn build(&self, filter: &str) -> Children {
        let fuzzy = fuzzy::Fuzzy::new();
        match self {
            Data::Commands(commands) => fuzzy.filter(filter, commands),
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
    let input_id = input.id();
    let mut window = Root(WidgetRef::new(
        Stack::rows(input.with_next_focus(input_id).and(Stack::rows(programs))),
    ))
    .into_window()
    .with_focused(|focused: &bool| {
        if !focused {
            exit(0)
        }
    });
    window.attributes.window_level = WindowLevel::AlwaysOnTop;
    window.attributes.resizable = false;
    window.attributes.title = "Starters".to_owned();
    window.attributes.transparent = true;
    // window.attributes.app_name = Some("de.modprog.starters".into());
    window.run().unwrap();
    // },
}

#[derive(Debug)]
struct Root(WidgetRef);

impl Widget for Root {
    fn redraw(&mut self, context: &mut gooey::context::GraphicsContext<'_, '_, '_, '_, '_>) {
        let color = context.theme().surface.color.with_alpha(u8::MAX - 50);
        context.gfx.fill(color);
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
}
