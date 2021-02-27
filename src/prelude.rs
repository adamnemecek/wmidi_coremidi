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
    MIDIAccess,
    MIDIEvent,
    MIDIInput,
    MIDIInputMap,
    MIDINotification,
    MIDIOutput,
    MIDIOutputMap,
    // MIDIPacket,
    MIDIPacketListIterator,
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortID,
    MIDIPortKind,
    MIDIPortMap,
    StateChangeFn,
};
