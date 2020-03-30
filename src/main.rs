use interactive_paper::{parser, QPaperBuilder, Builder, QuestionPaper, question_paper};
use std::sync::mpsc;
use std::thread;
use parser::{ProcessResult, Sink, Tokenizer, TokenizerResult, xml_content::XmlContent, interface::Tag};

use mpsc::{Sender, Receiver};

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use question_paper::{Read, Reference, Intent};

fn main() -> std::io::Result<()>{
    let xml_content = include_str!("/home/shaddy/Documents/shad/xml/qa_paper.xml");
    let mut lines = vec!(xml_content.to_string());

    let (tx, rx) = mpsc::channel();

    // timing
    let now = std::time::Instant::now();

    Tokenizer::tokenize(lines, Sink::new(tx));

    let handle = thread::spawn(move || {
        build_question_paper(rx)
    });

    let mut qpaper = handle.join().unwrap();
    let d = now.elapsed();
    let dt = d.as_secs() ;


    // read an intent for question 5
    let reference = Reference::Start(1);
    let intent = Intent::ReadIntent(Read::Question(reference));

    let response = qpaper.resolve_intent(intent);

    println!("Resolved {:#?}", response);


    Ok(())

}

// build a tree
fn build_question_paper(rx: Receiver<Tag>) -> QuestionPaper {
    let mut builder = QPaperBuilder::new();

    for tag in rx {
        builder.process_tag(tag);
    }

    builder.end()
}
