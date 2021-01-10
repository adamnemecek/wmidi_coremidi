use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::collections::HashMap<u32, T>,
}

impl<T: MIDIPort> MIDIPortMap<T> {}

impl MIDIPortMap<MIDIOutput> {
    pub fn new() -> Self {
        todo!()
    }
}
