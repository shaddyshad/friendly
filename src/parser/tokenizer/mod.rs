use std::borrow::Cow::{self, Borrowed};

mod states;
#[macro_use]
mod small_charset;

use super::xml_content::{XmlContent, FromSet, NotFromSet, SetResult};
use super::interface::{Token, Tag, TagName};
use super::interface::{TagKind, Attribute, SinkResult};
use super::sink::{Sink};

use TagKind::{StartTag, EndTag};
use Token::{ParseError, TagToken};

use std::mem::replace;
use states::States;
pub use small_charset::SmallCharSet;

/// Step function return type
#[derive(Debug, Eq, PartialEq)]
pub enum ProcessResult {
    Continue,
    Suspend
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenizerResult {
    Success,
    Done
}

pub struct Tokenizer {
    state: States,
    current_char: char,
    current_line: u64,
    current_tag_name: String,
    current_tag_attrs: Vec<Attribute>,
    current_tag_self_closing: bool,
    current_attr_name: String,
    current_attr_value: String,
    passage: String,
    sink: Sink,
    reconsume: bool,
    current_tag_kind: TagKind,
    last_start_tag: Option<TagName>
}

impl Tokenizer {
    pub fn new(sink: Sink) -> Self {
        Tokenizer {
            sink,
            state: States::Document,
            current_char: '\0',
            current_line: 1,
            current_tag_name: String::new(),
            current_attr_value: String::new(),
            passage: String::new(),
            current_tag_attrs: vec![],
            current_tag_self_closing: false,
            current_attr_name: String::new(),
            reconsume: false,
            current_tag_kind: TagKind::StartTag,
            last_start_tag: None
        }
    }
    fn bad_char_error(&mut self) {
        let msg = format!(
            "Bad character. Saw {} in state {:?} on line {}",
            self.current_char,
            self.state,
            self.current_line
        );

        self.emit_error(Cow::from(msg));
    }

    fn emit_error(&mut self, error: Cow<'static, str>) {
        self.process_and_continue(ParseError(error));
    }

    fn process_token(&mut self, token: Token) -> SinkResult {
        self.sink.process_token(token, self.current_line)
    }

    // process and continue
    fn process_and_continue(&mut self, token: Token) {
        assert!(
            matches!(
                self.process_token(token),
                SinkResult::Continue
        ))
    }

    fn discard_tag(&mut self) {
        self.current_tag_name.clear();
        self.current_tag_attrs = vec![];
        self.current_tag_self_closing = false;
    }

    // create a tag
    fn create_tag(&mut self, kind: TagKind, name: String) {
        self.discard_tag();
        self.current_tag_name = name;
        self.current_tag_attrs = vec![];
        self.current_tag_kind = kind;
    }

    // create an attribute value
    fn emit_attribute_name(&mut self, name: String){
        self.current_attr_name = name;
    }

    fn emit_attribute_value(&mut self, value: String){
        self.current_attr_value = value;
    }

    fn emit_attribute(&mut self){
        let attribute = Attribute {
            name: self.current_attr_name.clone(),
            value: self.current_attr_value.clone()
        };

        // push them to attrs
        self.current_tag_attrs.push(attribute);

        self.current_attr_name.clear();
        self.current_attr_value.clear();
    }

    fn emit_passage(&mut self, passage: String) {
        self.passage.push_str(&passage);
    }

    fn emit_tag_name(&mut self, name: String){
        self.current_tag_name = name;
    }

    fn set_self_closing(&mut self){
        self.current_tag_self_closing = true;
    }

    // emit a token
    fn emit_tag(&mut self){
        self.finish_attribute();

        let name = TagName::new(&self.current_tag_name);
        self.current_tag_name.clear();

        match self.current_tag_kind {
            StartTag => {
                self.last_start_tag = Some(name.clone());
            },
            EndTag => {
                if !self.current_tag_attrs.is_empty(){
                    self.emit_error(Borrowed("Attributes on an end tag"));
                }

                if self.current_tag_self_closing {
                    self.emit_error(Borrowed("Self-closing end tag"));
                }
            }
        }

        let value = if self.passage.is_empty(){
            None
        }else{
            Some(self.passage.clone())
        };

        self.passage.clear();

        let token = TagToken(Tag {
            kind: self.current_tag_kind,
            attributes: replace(&mut self.current_tag_attrs, vec![]),
            name,
            is_self_closing: self.current_tag_self_closing,
            value
        });

        self.current_tag_self_closing = false;


        self.process_token(token);
    }

    fn finish_attribute(&mut self) {
        if self.current_attr_name.is_empty() {
            return;
        }

        // check the attributes
        self.emit_attribute();

    }

    fn pop_except_from(&mut self, input: &mut XmlContent, set: SmallCharSet) -> Option<SetResult> {
        // Slow path for edge cases
        if self.reconsume {
            return self.get_char(input).map(|x| FromSet(x))
        }

        let d = input.pop_from_set(set);

        match d {
            Some(FromSet(c)) => self.get_preprocessed_char(c, input).map(|x| FromSet(x)),
            _ => d
        }
    }

    fn get_char(&mut self, input: &mut XmlContent) -> Option<char> {
        if self.reconsume {
            self.reconsume = false;
            Some(self.current_char)
        }else{
            input.next()
                .and_then(|c| self.get_preprocessed_char(c, input))
        }
    }

    /// Get the next input character 
    fn get_preprocessed_char(&mut self, mut c: char, input: &mut XmlContent) -> Option<char> {
        if c == '\n' {
            self.current_line += 1;
        }

        self.current_char = c;
        Some(c)
    }
}

/// Shorthand for common operations
macro_rules! shorthand (
    (  $me:ident  : error                                       )   => ( $me.bad_char_error();                );
    (  $me:ident  : create_tag  $kind:ident  $c:ident           )   => ( $me.create_tag($kind, $c);           );
    (  $me:ident  : emit_passage $passage:ident                 )   => ( $me.emit_passage($passage);          );
    (  $me:ident  : emit_tag                                    )   => ( $me.emit_tag();                      );
    (  $me:ident  : emit_attribute                              )   => ( $me.emit_attribute();                );
    (  $me:ident  : emit_tag_name $name:ident             )   => ( $me.emit_tag_name($name);      );
    (  $me:ident  : emit_self_closing                           )   => ( $me.set_self_closing();               );
);

/// A little DSl for our state machine 
macro_rules! go(
    // methods to call methods in self
    (  $me:ident : $a:tt                            ; $($rest:tt)*  ) => ({  shorthand!($me: $a);              go!($me: $($rest)*);     });
    (  $me:ident : $a:tt $b:tt                      ; $($rest:tt)*  ) => ({  shorthand!($me: $a $b);              go!($me: $($rest)*);     });
    (  $me:ident : $a:tt $b:tt  $c:tt               ; $($rest:tt)*  ) => ({ shorthand!($me: $a $b $c);        go!($me: $($rest)*);      });

    // macros to advance the state
    (  $me:ident :     to $s:ident                                  ) => ({ $me.state = States::$s; return ProcessResult::Continue; });
);

macro_rules! unwrap_or_else(
    ($opt:expr, $else_block:block) => {
        match $opt {
            None => $else_block,
            Some(x) => x
        }
    }
);

macro_rules! unwrap_or_return(
    ($opt:expr, $retval:expr) => {
        unwrap_or_else!($opt, {return $retval; })
    }
);

macro_rules! pop_from_set(
    (  $me:expr, $input:expr, $set:expr) => (
        unwrap_or_return!($me.pop_except_from($input, $set), ProcessResult::Suspend)
    )
);


impl Tokenizer {

    pub fn tokenize(input: Vec<String>, sink: Sink) {
        let mut tok = Tokenizer::new(sink);
        let mut xml_content = XmlContent::new();

        for chunk in input{
            xml_content.push_back(chunk);
            let _ = tok.feed(&mut xml_content);
        }
        
        tok.end();
    }

    pub fn feed(&mut self, xml_content: &mut XmlContent) -> TokenizerResult {
        if xml_content.is_empty(){
            return TokenizerResult::Done;
        }

        self.process(xml_content)
    }

    /// Signal the end of input
    pub fn end(&mut self){
        self.sink.end();
    }
    //run the stat e machine
    pub fn process(&mut self, input: &mut XmlContent) -> TokenizerResult{
        loop {
            match self.step(input) {
                ProcessResult::Continue => (),
                ProcessResult::Suspend => break,
            }
        }

        TokenizerResult::Success
    }
    pub fn step(&mut self, input: &mut XmlContent) -> ProcessResult{
        // println!("{:#?}", &self.state);
        match self.state {
            States::Document => loop{
                // read a token
                let set = small_char_set!(b'<' b' ' b'\n' b'\t' b'>' b'\0' b'?');
                match pop_from_set!(self, input, set) {
                    FromSet('<') => go!(self: to TagOpen),
                    FromSet(' ') | FromSet('\n') | FromSet('\t') => (),
                    FromSet('>') => self.emit_tag(),
                    FromSet('\0') => return ProcessResult::Suspend,
                    FromSet('?') => go!(self: to ProcessingInstruction),
                    NotFromSet(c) => {
                        
                        go!(self: emit_passage c; to Passage)
                    },
                    _ => {
                        return ProcessResult::Suspend
                    }
                }
            },
            States::TagOpen => loop {
                //read a token from this set
                let set = small_char_set!(b'?' b'/' b'>' b' ');

                match pop_from_set!(self, input, set){
                    FromSet('/') => go!(self: to StartClosingTag),
                    FromSet('>') => go!(self: emit_tag; to Document),
                    FromSet(' ') => go!(self: to BeforeAttributeName),
                    FromSet('?') => go!(self: to ProcessingInstruction),
                    NotFromSet(c) => self.create_tag(StartTag, c) ,
                    _ => return ProcessResult::Suspend
                }
            },
            States::ProcessingInstruction => loop {
                let set = small_char_set!(b'>' b' ');

                match pop_from_set!(self, input, set){
                    FromSet('>') => go!(self: emit_tag; to Document),
                    FromSet(' ') => go!(self: to BeforeAttributeName),
                    NotFromSet(c) => go!(self: emit_tag_name c; to TagName),
                    _ => return ProcessResult::Suspend
                }
            },
            States::StartClosingTag => loop {
                let set = small_char_set!(b'>');

                match pop_from_set!(self, input, set){
                    FromSet('>') => go!(self: emit_self_closing; emit_tag; to Document),
                    NotFromSet(c) => go!(self: create_tag EndTag c; to Document),
                    _ => return ProcessResult::Suspend
                }
            },
            States::TagName => loop {
                let set = small_char_set!(b'/' b' ' b'>' b'\n' b'\t');

                match pop_from_set!(self, input, set) {
                    FromSet('/') => self.current_tag_self_closing = true,
                    FromSet('>') => go!(self: emit_tag; to Document),
                    FromSet(' ') | FromSet('\n') | FromSet('\t') => go!(self: to BeforeAttributeName),
                    _ => return ProcessResult::Suspend
                }
            },
            States::BeforeAttributeName => loop {
                let set = small_char_set!(b'=' b'/' b'>' b'?' b' ');

                match pop_from_set!(self, input, set) {
                    FromSet('=') => go!(self: to StartAttributeValue),
                    FromSet('/') => go!(self: to StartClosingTag),
                    FromSet('>') => go!(self: emit_tag; to Document),
                    FromSet('?') => go!(self: to ProcessingInstruction),
                    FromSet(' ') => (),
                    NotFromSet(c) => self.emit_attribute_name(c),
                    _ => return ProcessResult::Suspend
                }
            },
            States::StartAttributeValue => loop {
                match input.next(){
                    Some('"') | Some('\'') => go!(self: to AttributeValue),
                    _ => go!(self: error; to Document)
                }
            },
            States::AttributeValue => loop {
                let set = small_char_set!(b'"'  b'\'' b' ');

                match pop_from_set!(self, input, set){
                    FromSet('"') | FromSet('\'') => go!(self: emit_attribute; to BeforeAttributeName),
                    FromSet(' ') => go!(self: to BeforeAttributeName),
                    NotFromSet(c) => self.emit_attribute_value(c),
                    _ => return ProcessResult::Suspend
                }
            },
            States::Passage => loop {
                let set = small_char_set!(b'<');
                match pop_from_set!(self, input, set) {
                    // passage completed
                    FromSet('<') => {
                        self.state = States::TagOpen;
                        return ProcessResult::Continue;
                    },
                    NotFromSet(c) => {
                        self.emit_passage(c);
                        return ProcessResult::Continue;
                    },
                    _ => return ProcessResult::Suspend
                }
            }
        }
    }
}

