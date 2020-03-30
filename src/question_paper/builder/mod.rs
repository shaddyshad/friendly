pub use crate::{TagToken, Tag, parser::interface::TagName};
use std::borrow::Cow::{self, Borrowed};
use super::{interface, QuestionPaper};
mod section_builder;
use std::mem::replace;

use section_builder::SectionBuilder;

pub use interface::{NodeData, Node, Builder, SectionData, QuestionData};


/// Builder modes controll how the builder interprets an input tag
#[derive(Debug, Clone, Copy)]
enum Modes {
    Root,
    Section,
}

#[derive(Debug)]
pub struct QPaperBuilder {
    errors: Vec<Cow<'static, str>>,
    nodes: Vec<Node>,
    mode: Option<Modes>,
    instructions: Vec<String>,
    section_builder: SectionBuilder
}


impl QPaperBuilder {
    pub fn new() -> Self {
        QPaperBuilder {
            errors: vec![],
            nodes: vec![],
            mode: None,
            instructions: vec![],
            section_builder: SectionBuilder::new()
        }
    }


    /// Process a tag in the current mode
    fn process_in_mode(&mut self, tag: Tag){
        if let Some(mode) = self.mode {
            match mode {
                Modes::Root => {
                    // process a meta tag
                    if tag.is_instructions(){
                        if tag.is_end_tag(){
                            self.process_instruction_text(tag);
                        }
                    }
    
                },
                Modes::Section => {
                    // process a section block
                    if tag.is_section(){
                        if tag.is_end_tag(){
                            self.insert_section();
                        }
                    }else{
                        self.section_builder.process_tag(tag);
                    }
                }
                _ => ()
            }
        }
        
    }

    /// Swaps the mode and returns the previous mode
    fn update_mode(&mut self, tag: &Tag) -> Option<Modes> {
        let prev_mode = self.mode;

        let mode = if tag.is_root(){
            Some(Modes::Root)
        }else if tag.is_section(){
            Some(Modes::Section)
        }else {
            prev_mode
        };

        self.mode = mode;

        prev_mode
    }

    // process a document tag
    fn process_document(&mut self, tag: Tag){
        let is_self_closing = match tag {
            Tag {
                is_self_closing: true,
                kind: StartTag,
                ..
            } => true,
            _ => false
        };

        if is_self_closing {
            self.emit_error(Borrowed("Document node cannot must be self closing"));
        }else{
            // append a document node
            if tag.is_start_tag(){
                self.append(NodeData::Document, None, None);
            } 
        }
    }


    // handle errors
    fn emit_error(&mut self, error: Cow<'static, str>){
        self.errors.push(error);
    }

    // append a node into this nodes array
    fn append(
        &mut self,
        data: NodeData,
        parent: Option<usize>,
        prev: Option<usize>
    ) -> usize {
        let index = self.nodes.len();

        // insert a navigator node
        self.nodes.push(Node {
            data,
            index,
            parent,
            prev,
            next: None,
            first_child: None,
            last_child: None,
        });

        if let Some(parent) = parent {
            let mut parent = &mut self.nodes[parent];

            if parent.first_child.is_none(){
                parent.first_child = Some(index);
            }

            parent.last_child = Some(index);
        }

        if let Some(prev) = prev {
            self.nodes[prev].next = Some(index);
        }

        index
    }
}

impl Builder for QPaperBuilder {
    type Item = QuestionPaper;

    fn process_tag(&mut self, tag: Tag) {
        if tag.is_document(){
            return self.process_document(tag);
        }

        // update the mode
        self.update_mode(&tag);
        
        self.process_in_mode(tag);
    }

    fn end(&mut self) -> Self::Item {
        let total = self.nodes.len();
        let nodes = replace(&mut self.nodes, vec![]);


        QuestionPaper::new(nodes, total)
    }
}


impl QPaperBuilder {
    // add an instructions to this document
    fn process_instruction_text(&mut self, tag: Tag){
        assert!(tag.is_end_tag());

        // extract the value from the tag
        if let Some(value) = tag.value(){
            self.instructions.push(value);
        }      
    }

    // insert a section
    fn insert_section(&mut self) {
        let section = self.section_builder.end();

        // append the section and its children
        let mut prev = None;

        let parent = self.append(NodeData::Section(section.section), Some(0), None);

        for question in section.questions{
            prev = Some(self.append(NodeData::Question(question), Some(parent), prev));
        }

        self.section_builder.clear();
    }
}