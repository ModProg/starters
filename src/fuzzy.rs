use executable_finder::Executable;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub trait Item {
    fn name(&self) -> String;
}

impl Item for Executable {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<T: Item> Item for &T {
    fn name(&self) -> String {
        (*self).name()
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

    pub fn filter<T: Item>(
        &self,
        filter: &str,
        options: impl IntoIterator<Item = T>,
    ) -> impl Iterator<Item = T> {
        let mut it: Vec<_> = options
            .into_iter()
            .filter_map(|elem| {
                self.fuzzy_finder
                    .fuzzy_match(&elem.name(), filter)
                    .map(|score| (elem, score))
            })
            .collect();

        it.sort_by_key(|e| -e.1);
        it.into_iter().map(|e| e.0)
    }
}
