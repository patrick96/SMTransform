use std::collections::HashSet;

pub struct VariableGenerator {
    taken: HashSet<String>,
    counter: i32,
}

impl VariableGenerator {
    pub fn new() -> Self {
        Self {
            taken: HashSet::new(),
            counter: 0,
        }
    }

    pub fn reserve<'a>(&mut self, names: impl IntoIterator<Item = &'a String>) {
        self.taken.extend(names.into_iter().cloned());
    }

    pub fn generate(&mut self) -> String {
        loop {
            let name = format!("v{}", self.counter);
            self.counter += 1;

            if !self.taken.contains(&name) {
                self.taken.insert(name.clone());
                return name;
            }
        }
    }
}
