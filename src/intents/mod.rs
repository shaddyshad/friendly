use crate::question_paper::intents;
mod resolvers;

use resolvers::{IntentResolver, Errors};
use intents::{Intent, Reference};

pub async fn resolve(input: &str) -> Result<Vec<Intent>, Errors> {
    let mut intent_resolver = IntentResolver::new();
    intent_resolver.resolve_input(input).await
}