mod builder;
mod interface;

pub use builder::{QPaperBuilder, Builder};

use interface::Node;

#[derive(Debug, Clone)]
pub struct QuestionPaper {
    pub nodes: Vec<Node>
}

impl QuestionPaper {
    pub fn new() -> Self {
        QuestionPaper {
            nodes: vec![]
        }
    }
}