pub trait Analyzer {
    fn analyze<'a>(&self, text: &'a str) -> impl Iterator<Item = &'a str>;
}

pub struct StandardAnalyzer {}

impl StandardAnalyzer {
    pub fn new() -> Self {
        StandardAnalyzer {}
    }
}

impl Analyzer for StandardAnalyzer {
    fn analyze<'a>(&self, text: &'a str) -> impl Iterator<Item = &'a str> {
        text.split(" ")
    }
}
