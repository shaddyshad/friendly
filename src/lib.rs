
pub mod parser;
pub mod question_paper;
pub mod intents;


pub use parser::interface::{Tag, Token::TagToken};
pub use question_paper::{QPaperBuilder, Builder, QuestionPaper};
pub use intents::resolve;