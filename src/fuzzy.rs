use std::process::exit;

use clonable_command::Command;
use cushy::{widget::WidgetList, widgets::Button};
use executable_finder::Executable;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub enum Action {
    Command(Command),
}

pub trait Item {
    fn name(&self) -> String;
    fn action(&self) -> Action;
}

impl Item for Executable {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn action(&self) -> Action {
        Action::Command(Command::new(&self.path))
    }
}

impl<T: Item> Item for &T {
    fn name(&self) -> String {
        (*self).name()
    }

    fn action(&self) -> Action {
        (*self).action()
    }
}
pub struct Fuzzy {
    fuzzy_finder: SkimMatcherV2,
}

impl Fuzzy {
    pub fn new() -> Self {
        Self {
            fuzzy_finder: SkimMatcherV2::default().use_cache(true),
        }
    }

    pub fn filter<T: Item>(&self, filter: &str, options: impl IntoIterator<Item = T>) -> WidgetList {
        let mut this: Vec<_> = options
            .into_iter()
            .filter_map(|elem| {
                self.fuzzy_finder
                    .fuzzy_match(&elem.name(), filter)
                    .map(|score| (elem, score))
            })
            .collect();

        this.sort_by_key(|e| -e.1);

        this.iter()
            .take(20)
            .map(|(it, _)| {
                // TODO highlight matched characters
                let action = it.action();
                Button::new(it.name()).on_click(move |_| match &action {
                    Action::Command(command) => {
                        if let Err(e) = command.spawn() {
                            eprintln!("Error spawning command: {e:#}")
                        };
                        exit(0);
                    }
                })
            })
            .collect()
    }
}
