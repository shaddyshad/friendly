use super::{Builder, Tag, TagName, QuestionData, SectionData};
use std::mem::replace;
/// builder modes for a setion
#[derive(Debug)]
enum Modes {
    Question,
    Meta
}

#[derive(Debug)]
pub struct Section {
    pub section: SectionData,
    pub questions: Vec<QuestionData>
}

/// Builder for a section block
#[derive(Debug)]
pub struct SectionBuilder {
    num_of_questions: u32,
    current_question: u32,
    mode: Option<Modes>,
    current_page: u32,
    last_page_name: Option<TagName>,
    questions: Vec<QuestionData>,
    current_section_name: String
}

impl SectionBuilder {
    pub fn new() -> Self {
        SectionBuilder {
            num_of_questions: 0,
            current_question: 1,
            mode: None,
            current_page: 1,
            last_page_name: None,
            questions: vec![],
            current_section_name: String::new()
        }
    }

    pub fn clear(&mut self){
       self.mode = None;
       self.num_of_questions = 0; 
       self.questions.clear();
    }


    fn process_question(&mut self, tag: Tag) {
        if tag.is_end_tag(){
            if tag.is_question(){
                self.current_question += 1;
                self.num_of_questions += 1;

                let question = tag.value().unwrap();

                // create a question data
                let question_data = QuestionData{
                    question,
                    page_number: self.current_page,
                    question_number: self.current_question
                };

                self.questions.push(question_data);
            }
        }
    }

    // insert a section meta tag
    fn get_section_data(&self) -> SectionData {
        SectionData {
            section_name: self.current_section_name.clone(),
            num_of_questions: self.num_of_questions,
            num_of_marked: 0,
            num_of_skipped: 0,
            num_of_attempted: 0,
            num_of_remaining: 0
        }
    }
}

impl Builder for SectionBuilder {
    type Item = Section;

    fn process_tag(&mut self, tag: Tag) {
        if tag.is_item(){
            if tag.is_start_tag(){
                if tag.is_start_tag(){
                    self.mode = Some(Modes::Question);
                }
            }
        }

        // incrememnt the page
        if tag.is_page(){
            if tag.is_end_tag(){
                // compare with the last page tagname
                match self.last_page_name {
                    Some(ref lp) => {
                        if lp != tag.name(){
                            self.current_page += 1;
                            self.last_page_name = Some(tag.name().clone());
                        }
                    },
                    _ => {
                        self.current_page += 1;
                        self.last_page_name = Some(tag.name().clone())
                    }
                }
                
            }
        }

        // set the section name
        if tag.is_section_name(){
            if tag.is_end_tag(){
                let name = tag.value().unwrap();

                self.current_section_name = name.clone();
            }
        }

        if let Some(ref mode) = self.mode {
            match mode {
                Modes::Question => {
                    self.process_question(tag);
                },
                _ => ()
            }
        }        
    }



    fn end(&mut self) -> Self::Item {
        Section {
            section: self.get_section_data(),
            questions: replace(&mut self.questions, vec![])
        }
    }
}
