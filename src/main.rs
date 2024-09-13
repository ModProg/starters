use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};

use clap::Parser;
use cushy::kludgine::app::winit::keyboard::{KeyCode, PhysicalKey};
use cushy::value::Dynamic;
use cushy::value::Source;
use cushy::widget::{MakeWidget, WidgetList, IGNORED};
use cushy::widgets::{Input, Stack};
use cushy::{kludgine::app::winit::window::WindowLevel, Run};
use executable_finder::{executables, Executable};

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
    fn build(&self, filter: &str) -> WidgetList {
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
            PhysicalKey::Code(KeyCode::Escape) => exit(0),
            _ => IGNORED,
        })
        .make_widget();
    let input_id = input.id();
    let window = Stack::rows(input.with_next_focus(input_id).and(Stack::rows(programs)))
        .into_window()
        .focused(Dynamic::new(true).with_for_each({
            let skip_initial_unfocus_because_ecton_is_wierd = AtomicBool::new(false);
            move |focused| {
                dbg!(focused);
                if !focused
                    && skip_initial_unfocus_because_ecton_is_wierd.swap(true, Ordering::Relaxed)
                {
                    exit(0)
                }
            }
        }))
        .window_level(WindowLevel::AlwaysOnTop)
        .resizable(false)
        .titled("Starters")
        .app_name("de.modprog.starters");
    window.run().unwrap();
}
