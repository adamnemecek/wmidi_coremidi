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
        write!(f, "MIDIPortID {{{:#x}}}", self.inner)
    }
}

impl std::fmt::Debug for MIDIPortID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIDIPortID {{{:#x}}}", self.inner)
    }
}

pub trait MIDIPort: Eq + Clone + std::hash::Hash + std::fmt::Debug {
    fn id(&self) -> MIDIPortID;
    fn manufacturer(&self) -> &str;
    fn name(&self) -> &str;
    fn display_name(&self) -> &str;
    fn kind(&self) -> MIDIPortKind;
    fn connection(&self) -> MIDIPortConnectionState;
    fn open(&mut self);
    fn close(&mut self);

    fn on_state_change(&self) -> Option<StateChangeFn<Self>>;
    fn set_on_state_change(&mut self, on_state_change: Option<StateChangeFn<Self>>);
}

pub type StateChangeFn<T> = std::rc::Rc<dyn Fn(T) -> ()>;
