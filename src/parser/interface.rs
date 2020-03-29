use std::borrow::Cow;


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
}

// Token sink result
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SinkResult {
    Continue
}