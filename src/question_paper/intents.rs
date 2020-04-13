use super::{Node, Predicate, NodeData, Find};
use std::borrow::Cow;
use serde::Serialize;

// intents interface
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Read {
    Question(Reference),
    Section(Reference),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Write {
    Mark(Vec<Read>),
    Skip(Vec<Read>),
    Note(Vec<Read>, String)
}


/// Reference with their skip values
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reference {
    Start(i32),
    End(i32),
    Current(i32)
}

impl Reference {
    pub fn is_forward(&self) -> bool {
        let val = match self {
            Self::Start(c) => c,
            Self::Current(c) => c,
            Self::End(c) => c
        };

        val >= &0 
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Intent {
    ReadIntent(Read),
    WriteIntent(Write),
    Meta(MetaIntent)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MetaIntent {
    Skipped,
    Marked
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum WriteResult {
    Success(String),
    Error(Cow<'static, str>)
}

pub type ReadResult = Result<Node, Cow<'static, str>>;

#[derive(Debug, Clone, Serialize)]
pub enum IntentResult {
    Read(Result<NodeData, Cow<'static, str>>),
    Write(WriteResult),
    Meta(String)
}

/// Types that can be used to resolve read intents
pub trait Reader {
    fn resolve_read_intent(&mut self, read_intent: &Read) -> ReadResult;

    /// Resolve a question
    fn resolve_question(&mut self, reference: &Reference) -> ReadResult;

    /// Resolve a section
    fn resolve_section(&mut self, reference: &Reference) -> ReadResult;

    /// Resolve from a reference
    fn resolve_referece<P: Predicate>(&mut self, reference: &Reference, predicate: P) -> ReadResult;

    /// resolve a general predicate
    fn resolve<P: Predicate>(&mut self, predicate: P, prev: usize, skip: usize, reference: &Reference) -> ReadResult;

    /// Do a foward find
    fn find_next<P: Predicate>(&self, finder: Find<P>) -> ReadResult;

    /// Do a reverse find
    fn find_back<P: Predicate>(&self, finder: Find<P>) -> ReadResult;

}

/// Types that can be used to resolve write intents
pub trait Writer {
    fn resolve_write_intent(&mut self, write_intent: &Write) ->  WriteResult;

    // process a read intent and mark it for review
    fn mark_for_review(&mut self, read_intent: &Vec<Read>) -> WriteResult;

    /// Skip the component found on the read intent position
    fn skip(&mut self, read_intent: &Vec<Read>) -> WriteResult;

    /// take a note on the component found
    fn note(&mut self, read_intent: &Vec<Read>, note: String) -> WriteResult;

}