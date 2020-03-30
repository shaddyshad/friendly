use std::borrow::Cow;
use regex::Regex;

// tokens
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    ParseError(Cow<'static, str>),
    TagToken(Tag)
    
}

// token kinds
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TagKind {
    StartTag,
    EndTag
}

#[derive(Debug, Eq, PartialEq)] 
pub struct Attribute {
    pub name: String,
    pub value: String
}

#[derive(Debug, Eq, PartialEq)]
pub struct Tag {
    pub kind: TagKind,
    pub name: TagName,
    pub attributes: Vec<Attribute>,
    pub is_self_closing: bool,
    pub value: Option<String>
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TagName(pub String);

impl TagName {
    pub fn new(name: &str) -> Self {
        TagName(name.to_string())
    }

    pub fn get(&self) -> &str {
        &self.0
    }

}

// Token sink result
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SinkResult {
    Continue,
}

//check if is an answer
fn matches<'a>(name: &'a str, pattern: &'static str) -> bool {
    let re = Regex::new(pattern).unwrap();

    return re.is_match(name);
}

impl Tag {
    // get the name of this tag
    pub fn name(&self) -> &TagName {
        &self.name
    }
    // get the value from the tag
    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }
    // check if it is an opening tag
    pub fn is_start_tag(&self) -> bool{
        self.kind == TagKind::StartTag
    }

    pub fn is_end_tag(&self) -> bool {
        !self.is_start_tag()
    }

    // section_number holds the section name so we can replace
    pub fn is_section_name(&self) -> bool {
        matches(
            &self.name.get(),
            "section_number"
        )
    }
    //check if it is a document tag
    pub fn is_document(&self) -> bool {
        let doc_tag = "xml";
        
        matches(
            &self.name.get(),
            doc_tag
        )
    }

    pub fn is_root(&self) -> bool {
        matches(
            &self.name.get(),
            "root"
        )
    }

    pub fn is_question_number(&self) -> bool {
        matches(
            &self.name.get(),
            r"^question_number$"
        )
    }

    pub fn is_section(&self) -> bool {
        matches(
            &self.name.get(),
            "SECTION_"
        )
    }

    pub fn is_question(&self) -> bool {
        matches(
            &self.name.get(),
            "^question$"
        )
    }

    pub fn is_item(&self) -> bool {
        matches(
            &self.name.get(),
            "item"
        )
    }

    pub fn is_instructions(&self) -> bool {
        matches(
            &self.name.get(),
            "instructions"
        )
    }

    // meta_data
    pub fn is_meta(&self) -> bool {
        matches(
            &self.name.get(),
            "meta_data"
        )
    }

    // new page
    pub fn is_page(&self) -> bool {
        matches(
            &self.name.get(),
            r"^page_\d{1}$"
        )
    }
}