pub struct SkipList {
    values: Vec<i32>,
}

impl SkipList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn insert(&mut self, value: i32) {
        if !self.values.contains(&value) {
            self.values.push(value);
            self.values.sort();
        }
    }

    pub fn contains(&self, value: i32) -> bool {
        self.values.contains(&value)
    }

    pub fn remove(&mut self, value: i32) -> bool {
        if let Some(index) = self.values.iter().position(|x| *x == value) {
            self.values.remove(index);
            true
        } else {
            false
        }
    }
}