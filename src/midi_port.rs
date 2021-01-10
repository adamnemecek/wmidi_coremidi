#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MIDIPortKind {
    Input,
    Output,
}

// impl From<coremidi::
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MIDIPortDeviceState {
    Disconnected,
    Connected,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MIDIPortConnectionState {
    Open,
    Closed,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MIDIPortID {
    inner: u32,
}

impl MIDIPortID {
    pub(crate) fn new(inner: u32) -> Self {
        Self { inner }
    }
}

pub trait MIDIPort: Eq + std::hash::Hash + std::fmt::Debug {
    fn id(&self) -> MIDIPortID;
}
