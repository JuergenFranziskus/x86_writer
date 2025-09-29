use std::ops::Index;

pub struct Labels {
    labels: Vec<String>,
}
impl Labels {
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }

    pub fn add(&mut self, label: impl Into<String>) -> Label {
        let index = self.labels.len();
        self.labels.push(label.into());
        Label(index)
    }
}
impl Index<Label> for Labels {
    type Output = str;
    fn index(&self, index: Label) -> &Self::Output {
        &self.labels[index.0]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Label(usize);
