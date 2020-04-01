use super::{Value, Errors};
use crate::question_paper::{Intent, Reference, Read};
use std::borrow::Cow;
use serde_json;
use serde::Deserialize;



/// Top LU intents
#[derive(Deserialize, Debug)]
pub enum TopIntents {
    Navigation,
    #[serde(alias = "boolean_position_check")]
    BooleanPositionCheck,
    #[serde(alias="mark_for_review")]
    MarkForReview
}

#[derive(Debug)]
enum Modes {
    Read, Write
}

/// Parses an intent response from LU into an intent that can be understood by the question paper
#[derive(Debug)]
pub struct IntentParser {
    mode: Option<Modes>
}


impl IntentParser {
    pub fn new() -> Self {
        IntentParser{
            mode: None
        }
    }

    pub fn parse(&mut self, intent: Value) -> Result<Intent, Errors> {
        let top_intent: TopIntents = serde_json::from_value(intent["top_intent"].clone()).unwrap_or(TopIntents::Navigation);

        // set the parsing mode
        self.set_mode(top_intent);

        println!("{:#?}", intent);
        
        Ok(Intent::ReadIntent(Read::Question(Reference::Start(1))))
    }

    fn set_mode(&mut self, top_intent: TopIntents) {
        let mode = match top_intent {
            TopIntents::Navigation | TopIntents::BooleanPositionCheck => Modes::Read,
            _ => Modes::Write
        };

        self.mode = Some(mode);
    }
}