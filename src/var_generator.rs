use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct VariableGenerator {
    taken: HashSet<String>,
    counter: i32,
}

impl VariableGenerator {
    pub fn reserve_one(&mut self, name: String) -> bool {
        self.taken.insert(name)
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
