use crate::Tag;

#[derive(Debug, Clone)]
pub enum NodeData {
    Document,
    Section(SectionData),
    Question(QuestionData),
    Instruction(String)
}

// Section data
#[derive(Debug, Clone)]
pub struct SectionData {
    pub num_of_questions: u32,
    pub num_of_attempted: u32,
    pub num_of_skipped: u32,
    pub num_of_marked: u32,
    pub num_of_remaining: u32,
    pub section_name: String
}

/// Question 
#[derive(Debug, Clone)]
pub struct QuestionData{
    pub question: String,
    pub question_number: u32,
    pub page_number: u32
}


/// A single document node
#[derive(Debug, Clone)]
pub struct Node {
    pub data: NodeData,
    pub index: usize,
    pub parent: Option<usize>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub first_child: Option<usize>,
    pub last_child: Option<usize>
}

/// A trait for all types that can be used to build different fragments
pub trait Builder {
    type Item;

    fn process_tag(&mut self, tag: Tag);

    fn end(&mut self) -> Self::Item;
}

