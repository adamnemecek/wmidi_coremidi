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
    pub(crate) inner: i32,
}

impl MIDIPortID {
    pub(crate) fn new(inner: i32) -> Self {
        Self { inner }
    }
}

impl From<i32> for MIDIPortID {
    fn from(a: i32) -> Self {
        Self::new(a)
    }
}

impl std::fmt::Display for MIDIPortID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIDIPortID {{{}}}", self.inner)
    }
}

impl std::fmt::Debug for MIDIPortID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIDIPortID {{{}}}", self.inner)
    }
}

pub trait MIDIPort: Eq + std::hash::Hash + std::fmt::Debug {
    fn id(&self) -> MIDIPortID;
    fn open(&self);
    fn close(&self);

    fn connection(&self) -> MIDIPortConnectionState;
}
