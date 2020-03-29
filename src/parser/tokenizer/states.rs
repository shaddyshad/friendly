
#[derive(Debug, Eq, PartialEq)]
pub enum States {
    Document,
    TagOpen,
    StartClosingTag,
    TagName,
    BeforeAttributeName,
    StartAttributeValue,
    AttributeValue,
    Passage,
    ProcessingInstruction
}