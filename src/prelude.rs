pub fn os_assert(status: i32) {
    if !status == 0 {
        panic!("error: {:?}", status);
    }
}

pub(crate) use crate::{
    MIDIClient,
    MIDIEndpoint,
};

pub use crate::{
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortKind,
};
