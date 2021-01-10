use crate::prelude::*;

pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::collections::HashMap<u32, T>,
}
