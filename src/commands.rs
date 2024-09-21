use std::process::{exit, Command};

use cushy::value::Source;
use executable_finder::{executables, Executable};

use crate::{buttons, fuzzy, Results};

pub struct Commands(Vec<Executable>);

impl Commands {
    pub fn new() -> Self {
        let mut data = executables().unwrap();
        data.sort();
        data.dedup();
        Self(data)
    }
}

impl Results for Commands {
    fn buttons(
        &self,
        filter: cushy::value::Dynamic<String>,
    ) -> cushy::value::Dynamic<cushy::widget::WidgetList> {
        let results = self.0.clone();
        filter.map_each(move |f| {
            buttons(fuzzy::Fuzzy::new().filter(f, &results).map(|e| {
                let path = e.path.clone();
                (e.name.clone(), move || {
                    _ = Command::new(&path).spawn();
                    exit(0)
                })
            }))
        })
    }

    fn execute(&self, filter: String) {
        if let Some(value) = fuzzy::Fuzzy::new().filter(&filter, &self.0).next() {
            _ = Command::new(&value.path).spawn();
        } else {
            let split = shlex::split(&filter)
                .unwrap_or_else(|| filter.split_whitespace().map(ToOwned::to_owned).collect());

            _ = Command::new(split.first().unwrap())
                .args(&split[1..])
                .spawn();
        }
        exit(0)
    }
}
