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
    MIDIAccess,
    MIDIInput,
    MIDIOutput,
    MIDIPacket,
    MIDIPacketListIterator,
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortID,
    MIDIPortKind,
    MIDIPortMap,
    MIDIEvent,
    // MIDIReceiver,
    // MIDISender,
};
