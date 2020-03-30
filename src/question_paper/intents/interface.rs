// intents interface
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Read {
    Question(Reference),
    Section(Reference),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Write {
    Mark(Read),
    Skip(Read)
}

/// Reference with their skip values
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reference {
    Start(i32),
    End(i32),
    Current(i32)
}

impl Reference {
    pub fn is_forward(&self) -> bool {
        let val = match self {
            Self::Start(c) => c,
            Self::Current(c) => c,
            Self::End(c) => c
        };

        val >= &0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Intent {
    ReadIntent(Read),
    WriteIntent(Write)
}