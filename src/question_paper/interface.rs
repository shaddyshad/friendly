use crate::Tag;
use super::QuestionPaper;
use std::borrow::Cow::{self, Borrowed};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum NodeData {
    Document,
    Section(SectionData),
    Question(QuestionData),
    Instruction(String)
}

impl NodeData {
    pub fn is_question(&self) -> bool {
        match self {
            Self::Question(ref c) => true,
            _ => false
        }
    }

    pub fn is_section(&self) -> bool {
        match self {
            Self::Section(ref c) => true,
            _ => false
        }
    }

}

// Section data
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SectionData {
    pub num_of_questions: u32,
    pub num_of_attempted: u32,
    pub num_of_skipped: u32,
    pub num_of_marked: u32,
    pub num_of_remaining: u32,
    pub section_name: String
}

/// Question 
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuestionData{
    pub question: String,
    pub question_number: u32,
    pub page_number: u32,
    pub marked: bool
}

impl Default for QuestionData {
    fn default() -> Self {
        QuestionData {
            question: String::new(),
            question_number: 0,
            page_number: 1,
            marked: false
        }
    }
}


/// A note can be taken on any node
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Note{
    pub index: usize,
    pub note: String
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

/// A type to index the nodes
pub struct NodeIndex<'a> {
    question_paper: &'a QuestionPaper,
    index: usize
}

impl<'a> NodeIndex<'a> {
    pub fn new(question_paper: &'a QuestionPaper, index: usize) -> Option<Self>{
        if index < question_paper.nodes.len(){
            Some(NodeIndex {question_paper, index})
        }else{
            None
        }
    }

    pub fn raw(&self) -> &Node{
        &self.question_paper.nodes[self.index()]
    }

    pub fn data(&self) -> &NodeData {
        &self.raw().data
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

/// A trait for all types that can be used to build different fragments
pub trait Builder {
    type Item;

    fn process_tag(&mut self, tag: Tag);

    fn end(&mut self) -> Self::Item;
}

/// A trait for all predicates to be applied on a node
pub trait Predicate {
    fn matches(&self, node: &NodeIndex) -> bool;

    fn and<T: Predicate>(self, other: T) -> And<Self, T> where Self:Sized {
        And(self, other)
    }
}

// and two predicates
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct And<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for And<A, B> {
    fn matches(&self, other: &NodeIndex) -> bool {
        self.0.matches(other) && self.1.matches(other)
    }
}


// predicates to search the document
pub mod predicates {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct QuestionPredicate;

    impl Predicate for QuestionPredicate {
        fn matches(&self, other: &NodeIndex) -> bool {
            other.data().is_question()
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct SectionPredicate;

    impl Predicate for SectionPredicate {
        fn matches(&self, other: &NodeIndex) -> bool {
            other.data().is_section()
        }
    }
}