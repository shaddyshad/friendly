mod builder;
mod interface;
mod intents;

use std::borrow::Cow::{Borrowed, self};

pub use builder::{QPaperBuilder, Builder};
use interface::{Node, Predicate, NodeIndex, NodeData, predicates};
pub use intents::{Read, Write, Reference, Intent};

use Reference::{Start, Current, End};

#[derive(Debug, Clone)]
pub struct QuestionPaper {
    pub nodes: Vec<Node>,
    prev_index: usize,
    last_index: usize
}

type IntentResult = Result<NodeData, Cow<'static, str>>;

impl QuestionPaper {
    pub fn new(nodes: Vec<Node>, last_index: usize) -> Self {
        QuestionPaper {
            nodes,
            prev_index:0,
            last_index
        }
    }

    // find a node on a certain predicate
    fn find<P: Predicate>(&self, predicate: P, next: usize, skip: usize) -> Find<P> {
        Find {
            question_paper: self,
            predicate,
            next,
            skip
        }
    }

    // return the nth node in this document
    pub fn nth(&self, index: usize) -> Option<NodeIndex> {
        NodeIndex::new(self, index)
    }

    // get the total number of nodes
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    // get the previous index
    pub fn prev_index(&self) -> usize {
        self.prev_index
    }

    pub fn last_index(&self) -> usize {
        self.last_index
    }

    // resolve a user intent
    pub fn resolve_intent(&mut self, intent: Intent) -> IntentResult {
        println!("{:#?}", &intent);
        match intent {
            Intent::ReadIntent(ref read_intent) => self.resolve_read_intent(read_intent),
            Intent::WriteIntent(ref write_intent) => self.resolve_write_intent(write_intent)
        }
    }

    /// Resolves a read intent
    fn resolve_read_intent(&mut self, read_intent: &Read) -> IntentResult {
        match read_intent {
            Read::Question(ref question) => self.resolve_question(question),
            Read::Section(ref section) => self.resolve_section(section)
        }
    }

    /// Resolve a write intent
    fn resolve_write_intent(&mut self, write_intent: &Write) -> IntentResult {
        Err(Borrowed("cannot resolve a write intent yet"))
    }

    /// Resolve a question
    fn resolve_question(&mut self, reference: &Reference) -> IntentResult {
        let predicate = predicates::QuestionPredicate;

        self.resolve_referece(reference, predicate)
    }

    /// Resolve a section
    fn resolve_section(&mut self, reference: &Reference) -> IntentResult {
        let predicate = predicates::SectionPredicate;

        self.resolve_referece(reference, predicate)
    }

    /// Resolve from a reference
    fn resolve_referece<P: Predicate>(&mut self, reference: &Reference, predicate: P) -> IntentResult {

        let (prev, skip) = match reference {
            Start(skip) => (0, *skip as usize),
            Current(skip) => (self.prev_index(), *skip as usize),
            End(skip) => (self.last_index(), *skip as usize)
            
        };
        

        self.resolve(predicate, prev, skip, reference)
    }

    fn resolve<P: Predicate>(&mut self, predicate: P, prev: usize, skip: usize, reference: &Reference) -> IntentResult {
        let finder = self.find(predicate, prev, skip);

        if reference.is_forward(){
            self.find_next(finder)
        }else{
            self.find_back(finder)
        }
    }

    /// Do a foward find
    fn find_next<P: Predicate>(&self, mut finder: Find<P>) -> IntentResult {
        if let Some(node) = finder.next(){
            Ok(node.data().clone())
        }else{
            Err(Borrowed("Could not find a next node"))
        }
    }

    /// Do a reverse find
    fn find_back<P: Predicate>(&self, mut finder: Find<P>) -> IntentResult {
        if let Some(node) = finder.next_back(){
            Ok(node.data().clone())
        }else{
            Err(Borrowed("Could not resolve a previous node"))
        }
    }

}

pub struct Find<'a, P:Predicate> {
    predicate: P,
    next: usize,
    question_paper: &'a QuestionPaper,
    skip: usize
}

impl <'a, P: Predicate> Iterator for Find<'a, P> {
    type Item = NodeIndex<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.question_paper.len(){
            let node = self.question_paper.nth(self.next).unwrap();

            self.next += 1;

            if self.predicate.matches(&node){
                if self.skip > 0 {  
                   
                    self.skip -= 1;
                }else{
                    return Some(node);
                }
            }
        }

        None
    }
}


impl<'a, P: Predicate> DoubleEndedIterator for Find<'a, P> {
    fn next_back(&mut self) -> Option<NodeIndex<'a>> {
        while self.next > 0 {
            let node = self.question_paper.nth(self.next).unwrap();

            self.next -= 1;

            if self.predicate.matches(&node) {
                return Some(node);
            }
        }

        None
    }
}