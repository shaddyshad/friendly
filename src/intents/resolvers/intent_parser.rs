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
        let mut intents = Vec::new();

        let mut prev = 0;
        for entity in &entities {
           match self.get_reference(entity, prev){
               Ok(reference) => {
                   if let Some(mode) = &self.mode {
                        match mode {
                            Modes::Read => intents.push(self.parse_read_intent(entity, reference)),
                            Modes::Write => intents.push(self.parse_write_intent(entity, reference))
                        }
                   }
                    
               },
               Err(err) => ()
           }
           prev += 1;
        }

        intents
    }
    // create a read intent
    fn parse_read_intent(&self, entity: &Entity, reference: Reference) -> Intent {
        let read_intent = self.parse_read(entity, reference);

        Intent::ReadIntent(read_intent)
    }

    // create a write inetnt
    fn parse_write_intent(&self, entity: &Entity, reference: Reference) -> Intent {
        let read_intent = self.parse_read(entity, reference);

        Intent::WriteIntent(Write::Skip(read_intent))
    }

    // parse a read query
    fn parse_read(&self, entity: &Entity, reference: Reference) -> Read {
        let entity = match entity.entity_type() {
            EntityType::Question => Read::Question(reference),
            EntityType::Section => Read::Section(reference)
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