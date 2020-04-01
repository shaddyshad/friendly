use crate::question_paper::Intent;
mod resolvers;
use std::borrow::Cow;

use resolvers::{IntentResolver, Errors};

pub async fn resolve(input: &str) -> Result<Intent, Errors> {
    let mut intent_resolver = IntentResolver::new();
    intent_resolver.resolve_input(input).await
}