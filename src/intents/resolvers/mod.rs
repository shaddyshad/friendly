use std::borrow::Cow;
use super::Intent;
mod http;
mod intent_parser;

use intent_parser::IntentParser;
pub use http::{HttpResolver, Value};

/// Intent parsing errors
#[derive(Debug)]
pub enum Errors {
    NetworkError(String),
    ParsingError,
    InvalidInput
}

type IntentResult = Result<Intent, Errors>;


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

        self.parser.parse(lu_response)
    }
}

