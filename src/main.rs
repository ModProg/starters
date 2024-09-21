use std::num::NonZero;
use std::process::exit;

use clap::Parser;
use commands::Commands;
use cushy::figures::units::Px;
use cushy::figures::{Size, UPx2D};
use cushy::kludgine::app::winit::keyboard::{KeyCode, PhysicalKey};
use cushy::styles::components::WidgetBackground;
use cushy::styles::ThemePair;
use cushy::value::Dynamic;
use cushy::value::Source;
use cushy::widget::{MakeWidget, WidgetList, IGNORED};
use cushy::widgets::{Button, Input, Stack};
use cushy::window::ThemeMode;
use cushy::{kludgine::app::winit::window::WindowLevel, Run};

mod commands;
mod fuzzy;

#[derive(Debug, Parser)]
enum Cmd {
    Commands,
    Windows,
    Apps,
}

trait Results: Send + Sync {
    fn buttons(&self, filter: Dynamic<String>) -> Dynamic<WidgetList>;
    fn execute(&self, filter: String);
}

pub fn buttons(
    entries: impl IntoIterator<Item = (String, impl Fn() + Send + Sync + 'static)>,
) -> WidgetList {
    entries
        .into_iter()
        .take(20)
        .enumerate()
        .map(|(i, (name, cb))| {
            // TODO highlight matched characters
            let button = Button::new(name).on_click(move |_| cb());
            if i == 0 {
                button.into_default().make_widget()
            } else {
                button.make_widget()
            }
        })
        .collect()
}

pub fn main() {
    let results: Box<dyn Results> = match Cmd::parse() {
        Cmd::Commands => Box::new(Commands::new()),
        Cmd::Windows => todo!(),
        Cmd::Apps => todo!(),
    };

    // let color_scheme = ColorScheme::default();
    let theme = ThemePair::default();

    let filter = Dynamic::new("".to_string());
    let buttons = results.buttons(filter.clone());

    let themed_mode = Dynamic::new(match dark_light::detect() {
        dark_light::Mode::Dark | dark_light::Mode::Default => ThemeMode::Dark,
        dark_light::Mode::Light => ThemeMode::Light,
    });

    themed_mode.for_each(|m| eprintln!("{m:?}")).persist();

    let input = Input::new(filter.clone())
        .on_key(move |key| match key.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => exit(0),
            PhysicalKey::Code(KeyCode::Enter) => {
                results.execute(filter.get());
                IGNORED
            }
            _ => IGNORED,
        })
        .with(
            &WidgetBackground,
            themed_mode.map_each(move |mode| {
                match mode {
                    ThemeMode::Light => theme.light,
                    ThemeMode::Dark => theme.dark,
                }
                .surface
                .color
            }),
        )
        .make_widget();
    let input_id = input.id();
    let window = Stack::rows(input.with_next_focus(input_id).and(Stack::rows(buttons)))
        .width(Px::new(300))
        .height(..Px::new(500))
        .themed_mode(themed_mode)
        .themed(theme)
        .into_window()
        .focused(Dynamic::new(true).with_for_each({
            let mut skipped_initial_unfocus_because_winit_is_wierd = false;
            move |focused| {
                if !focused {
                    if skipped_initial_unfocus_because_winit_is_wierd {
                        exit(0)
                    } else {
                        skipped_initial_unfocus_because_winit_is_wierd = true;
                    }
                }
            }
        }))
        // immediately "show" window to start capturing input
        .on_init(|window| {
            let mut surface =
                softbuffer::Surface::new(&softbuffer::Context::new(window).unwrap(), window)
                    .unwrap();
            surface
                .resize(NonZero::new(1).unwrap(), NonZero::new(1).unwrap())
                .unwrap();
            let buffer = surface.buffer_mut().unwrap();
            window.pre_present_notify();
            buffer.present().unwrap();
        })
        .inner_size(Dynamic::new(Size::upx(300, 1)))
        .window_level(WindowLevel::AlwaysOnTop)
        .transparent()
        .resizable(false)
        .titled("Starters")
        .app_name("de.modprog.starters");
    window.run().unwrap();
}
