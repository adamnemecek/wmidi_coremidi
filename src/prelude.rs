pub fn os_assert(status: i32) {
    if !status == 0 {
        panic!("error: {:?}", status);
    }
}

pub(crate) use crate::{
    MIDIClient,
    MIDIEndpoint,
    MIDINotification,
};

pub use crate::{
    MIDIPortID,
    MIDIAccess,
    MIDIInput,
    MIDIOutput,
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortKind,
    MIDIPortMap,

};
