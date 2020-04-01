use super::interface::{Token, SinkResult, Tag};
use std::borrow::Cow;
use Token::{ParseError, TagToken};
use std::sync::mpsc::Sender;

pub struct Sink {
    line_number: u64,
    errors: Vec<Cow<'static, str>>,
    tb: Sender<Tag>
}

impl Sink {
    pub fn new(tb: Sender<Tag>) -> Self {
        Sink {
            line_number: 1,
            errors: vec![],
            tb
        }
    }
    pub fn process_token(&mut self, token: Token, line_number: u64) -> SinkResult{
        if line_number != self.line_number {
            self.line_number = line_number;
        }


        let token = match token {
            ParseError(e) => {
                self.parse_error(e);
                return SinkResult::Continue;
            },
            TagToken(token) => TagToken(token)
        };

        self.process_to_completion(token)
    }

    fn parse_error(&mut self, error: Cow<'static, str>){
        self.errors.push(error)
    }


    fn process_to_completion(&mut self, token: Token) -> SinkResult {

        // process the tokens
        match token {
            TagToken(tag) => {
                self.tb.send(tag).unwrap()
            },
            _ => return SinkResult::Continue
        }

        return SinkResult::Continue;
    }

    pub fn end(&mut self){
        // drop the transmitter
        println!("Report error");
    }
}