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
