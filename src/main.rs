use interactive_paper::{parser, QPaperBuilder, Builder, QuestionPaper};
use std::sync::mpsc;
use std::thread;
use parser::{ProcessResult, Sink, Tokenizer, TokenizerResult, xml_content::XmlContent, interface::Tag};

use mpsc::{Sender, Receiver};

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

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

    let x = handle.join().unwrap();
    let d = now.elapsed();
    let dt = d.as_millis() ;

    println!("Took {:#?}", x);


    Ok(())

}

// build a tree
fn build_question_paper(rx: Receiver<Tag>) -> QuestionPaper {
    let mut builder = QPaperBuilder::new();
    let mut c = 10;

    for tag in rx {
        builder.process_tag(tag);
    }

    builder.end()
}
