
pub mod parser;
pub mod question_paper;
pub mod intents;
mod errors;


pub use parser::interface::{Tag, Token::TagToken};
use parser::{XmlContent, Sink, Tokenizer};
pub use question_paper::{QPaperBuilder, Builder, QuestionPaper, Intent, IntentResult};
pub use intents::resolve;

use std::sync::{Arc, RwLock};
use actix_multipart::Multipart;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use actix_web::{web, HttpRequest, HttpServer, get, post, HttpResponse, App, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::time::SystemTime;

use errors::Errors;
// for keeping safe multihteadable state
pub struct State ( Option<QuestionPaper> );

pub type StateData = Arc<RwLock<State>>;

impl State {
    pub fn new() -> Self {
        State (None)
    }

    pub fn handle_intents(&mut self, intents: Vec<Intent>) -> Result<IntentResult, Errors> {
        if self.0.is_none(){
            return Err(Errors::InternalError("No question paper has been initialized. Maybe you forgot to upload.".to_string()));
        }

        if let Some(ref mut question_paper) = self.0 {
            let mut r : 
                Vec<IntentResult> = intents.into_iter()
                                            .map(|intent| question_paper.resolve_intent(intent))
                                            .collect();

            if let Some(res) = r.pop(){
                return Ok(res);
            }else{
                return Err(Errors::InternalError("Could not pop anything, try again".to_string()));
            }
                
        }
        
        Err(Errors::InternalError("Could not resolve your request, try again".to_string()))
    }
}
// async function to resolve a user input
pub async fn resolve_intent(state: web::Data<StateData>, input: &str) -> Result<IntentResult, Errors> {
    let now = SystemTime::now();

    match resolve(input).await {
        Ok(intents) => {
            if let Ok(d) = now.elapsed(){
                let dt = d.as_micros();

                println!("Took {} ms to resolve an intent from LU.", dt);
            }
            let mut state = state.write().unwrap();

            match state.handle_intents(intents) {
                Ok(res) => Ok(res),
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
    
}
// async function to handle network upload
pub async fn upload(state: web::Data<StateData>, mut payload: Multipart) -> Result<String, Errors> {
    let mut content = String::new();

    // iterate over the multipart data 
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let _filename = content_type.get_filename().unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            content.push_str(&std::str::from_utf8(&data).unwrap().to_owned());
        }
    }

    // initialize the question paper with the content
    let question_paper = parse_content(&content);

    let mut state = state.write().unwrap();

    state.0 = Some(question_paper);

    Ok(format!("uploaded {} bytes", content.len()))
}

fn parse_content(content: &str) -> QuestionPaper{
    // create a new tokenizer and sink, start a thread to initialize all
    let (tx, rx) = mpsc::channel();

    tokenize(content, tx);

    // start a new thread to build the tree
    build_question_paper(rx)
}


// create a new thread to tokenzie content
fn tokenize(content: &str, tx: Sender<Tag>) {
    let mut input = Vec::new();
    input.push(content.to_string());

    thread::spawn(move || {
        Tokenizer::tokenize(input, Sink::new(tx));
    });
}

// a new thread to build a question paper
fn build_question_paper(rx: Receiver<Tag>) -> QuestionPaper{
    let handle = thread::spawn(move || {
        let mut builder = QPaperBuilder::new();

        for tag in rx {
            builder.process_tag(tag);
        }
        // finish processing and return the handler or and error
        builder.end()
    });

    handle.join().unwrap()
}