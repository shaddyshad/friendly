use std::borrow::Cow;
use super::{Intent, Reference};
mod http;
mod intent_parser;

use intent_parser::IntentParser;
pub use http::{HttpResolver};

pub type Value = serde_json::Value;
use serde::Deserialize;

pub use crate::errors::Errors;

type IntentResult = Result<Vec<Intent>, Errors>;


/// Main intent resolver
pub struct IntentResolver {
    resolver: HttpResolver,
    parser: IntentParser
}

impl IntentResolver{
    pub fn new() -> Self {
        IntentResolver {
            resolver: HttpResolver::new(),
            parser: IntentParser::new()
        }
    }

    pub async fn resolve_input(&mut self, input: &str) -> IntentResult {
        let lu_response = self.resolver.get(input).await?;

        let response = self.parser.parse(lu_response);

        Ok(response)
    }
}


#[derive(Deserialize, Debug)]
struct LuResponse {
    top_intent: TopIntents,
    #[serde(alias = "Entities")]
    entities: Vec<Entity>
}

#[derive(Deserialize, Debug)]
struct Entity {
    entity: EntityType,
    #[serde(alias = "CHILD")]
    children: Vec<EntityChild>
}

impl Entity {
    pub fn entity_type(&self) -> EntityType {
        self.entity
    }

    fn has_more_than_one_child(&self) -> bool {
        self.children.len() > 1
    }

    pub fn child(&self) -> Result<EntityChild, &'static str> {
        if self.has_more_than_one_child(){
            return Err("Entity has more than one child");
        }

        // get the top most 
       Ok( self.children[0].clone())
    }
}

#[derive(Deserialize, Debug, Clone)]
struct EntityChild {
    value: Value
}

#[derive(Deserialize, Debug, Clone)]
struct Relative {
    offset: i32,
    #[serde(alias = "Entities")]
    relative_to: String
}

impl EntityChild {
    pub fn is_object(&self) -> bool {
        self.value.is_object()
    }

    pub fn get_value(&self) -> i32 {
        if self.is_object(){
            // fetch the offset from the value
            let offset = self.value.clone();
            // read the offset

            let offset: i32 = offset["offset"].to_string().parse().unwrap();
            return offset;
        }
        
        let mut val = self.value.to_string();
        let val = val.replace('\"', "");
        
        val.parse::<i32>().unwrap()
    }

    pub fn get_reference(&self, prev: u32) -> Reference {
        let value = self.get_value();

        if prev > 0 {
            return Reference::Current(value);
        }

        // check if value has a reference
        if !self.is_object(){
            return Reference::Start(value);
        }

        let val = self.value.clone();

        let relative_to = &val["relativeTo"];

        if relative_to == "start"{
            return Reference::Start(value);
        }else if relative_to == "current"{
            return Reference::Current(value);
        }else{
            return Reference::End(value);
        }
    }
}

/// Top LU intents
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum TopIntents {
    Navigation,
    #[serde(alias = "boolean_position_check")]
    BooleanPositionCheck,
    #[serde(alias="mark_for_review")]
    MarkForReview
}

/// Entity types
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum EntityType {
    #[serde(alias = "section_number")]
    #[serde(alias = "section_ordinal")]
    #[serde(alias = "typeofnav_section")]
    Section,
    #[serde(alias = "question_number")]
    #[serde(alias = "section_number")]
    #[serde(alias = "typeofnav_question")]
    Question
}

impl LuResponse {

    pub fn is_read(&self)  -> bool {
        match self.top_intent {
            TopIntents::Navigation
            | TopIntents::BooleanPositionCheck => true,
            _ => false
        }
    }
}