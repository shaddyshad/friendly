use interactive_paper::{parser};
use std::sync::mpsc;
use std::thread;
use parser::{ProcessResult, Sink, Tokenizer, TokenizerResult, xml_content::XmlContent, interface::Tag};

use mpsc::{Sender, Receiver};

fn main() {
    let xml = include_str!("/home/shaddy/Documents/shad/xml/qa_paper.xml");
    let mut xml_content = XmlContent::new(xml);

    let (tx, rx) = mpsc::channel();

    // timing
    let now = std::time::Instant::now();
    let mut tokenizer = Tokenizer::new(Sink::new(tx));

    let handle = thread::spawn(move || {
        build_tree(rx);
    });

    match tokenizer.process(&mut xml_content){
        TokenizerResult::Success => {
            let d = now.elapsed();
            let dt = d.as_millis() ;

            println!("Took {:?} to finish ", dt);
        }
    }

    
    handle.join().unwrap();
}

// build a tree
fn build_tree(rx: Receiver<Tag>) {
    let mut tokens = rx.iter();

    while let Some(tag) = tokens.next(){
        println!("{:?}", tag);
    }
}
