use super::tokenizer::SmallCharSet;

#[derive(Debug)]
pub enum SetResult {
    FromSet(char),
    NotFromSet(String)
}

pub use SetResult::{FromSet, NotFromSet};

pub struct XmlContent {
    content: String
}

impl XmlContent {
    pub fn new(content: &str) -> Self {
        XmlContent {content: String::from(content)}
    }

    /// Pops and return either a single character from a given set
    /// or a buffer of character not within the set
    pub fn pop_from_set(&mut self, set: SmallCharSet) -> Option<SetResult> {
        if self.content.is_empty(){
            return None;
        }
        let n = set.nonmember_prefix_len(&self.content);

        if n > 0 {
            // not from set
            let out: String = self.content.drain(..n as usize).collect();

            return Some(NotFromSet(out));
        }else{
            let c = self.content.remove(0);

            return Some(FromSet(c));
        }
    }

    /// Retrieve the next character in the top of the buffer
    pub fn next(&mut self) -> Option<char> {
        if self.content.is_empty() {
            return None;
        }

        Some(self.content.remove(0))
    }
}