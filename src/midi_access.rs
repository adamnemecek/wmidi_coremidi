use crate::prelude::*;

pub struct MIDIAccess {
    client: MIDIClient,
}

impl MIDIAccess {
    pub fn new(name: &str) -> Self {
        Self {
            client: MIDIClient::new(name),
        }
    }
}
