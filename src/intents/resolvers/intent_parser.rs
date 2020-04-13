use super::{Value, Errors, LuResponse, Entity, EntityType};
use crate::question_paper::{Intent, Reference, Read, Write};
use std::borrow::Cow;
use serde_json;
use serde::Deserialize;



/// Response parser modes
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

    pub fn parse(&mut self, intent: LuResponse) -> Vec<Intent> {
        if intent.is_read(){
            self.mode = Some(Modes::Read);
        }else{
            self.mode = Some(Modes::Write);
        }

        // get the iterator of intents 
        self.process_intents(intent.entities)
    }

    fn process_intents(&mut self, entities: Vec<Entity>) -> Vec<Intent> {
        if let Some(Modes::Write) = &self.mode {
            self.create_write_intent(entities)
        }else{
            self.create_read_intent(entities)
        }
        
    }

    fn create_read_intent(&mut self, entities: Vec<Entity>) -> Vec<Intent> {
        // create an array of intents
        let mut intents = Vec::new();

        let mut prev = 0;
        for entity in &entities {
            let reference = self.get_reference(entity, prev).expect("could not compute the reference");

            intents.push(self.parse_read_intent(entity, reference));
           prev += 1;
        }

        intents
    }

    fn create_write_intent(&mut self, mut entities: Vec<Entity>) -> Vec<Intent> {
        // create a write intent array
        let top = entities.remove(0);
        println!("{:#?}", &entities);

        let mut ret = Vec::new();

        let mut reads = Vec::new();
        let mut prev = 0;

        for entity in entities {
            let reference = self.get_reference(&entity, prev).expect("could not get the reference");

            reads.push(self.parse_read(&entity, reference));
        }

        match top.entity_type() {
            EntityType::Mark => {
                // create a locator marked from the remaining items
                ret.push(
                    Intent::WriteIntent(Write::Mark(reads))
                )
            },
            EntityType::Skip => {
                ret.push (
                    Intent::WriteIntent(Write::Skip(reads))
                )
            }
            _ => ()
        }

        ret
    }
    // create a read intent
    fn parse_read_intent(&self, entity: &Entity, reference: Reference) -> Intent {

        let read_intent = self.parse_read(entity, reference);

        Intent::ReadIntent(read_intent)
    }

    // parse a read query
    fn parse_read(&self, entity: &Entity, reference: Reference) -> Read {
        let entity = match entity.entity_type() {
            EntityType::Question => Read::Question(reference),
            EntityType::Section => Read::Section(reference),
            _ => Read::Question(reference)
        };

        entity
    }

    fn get_reference(&mut self, entity: &Entity, prev: u32) -> Result<Reference, &'static str> {
        match entity.child(){
            Ok(child) => Ok(child.get_reference(prev)),
            Err(e) => Err(e)
        } 
    }

}