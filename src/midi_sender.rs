// similar to std::sync::mpsc::Receiver
use crate::prelude::*;
// use crate::MIDIPacket;

pub struct MIDISender {
    // inner: std::sync::mpsc::Sender<MIDIPacket>,
    endpoint: MIDIEndpoint,
    port: coremidi_sys::MIDIPortRef,
}

unsafe impl Send for MIDISender {}
impl !Sync for MIDISender {}

#[inline]
#[allow(non_snake_case)]
fn CreateMIDIPacket(timestamp: u64, data: &[u8]) -> coremidi_sys::MIDIPacket {
    let mut self_ = coremidi_sys::MIDIPacket {
        length: 0,
        timeStamp: 0,
        data: [0; 256],
        __padding: [0; 2],
    };

    // self_.data.copy_from_slice(data);
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), self_.data.as_mut_ptr(), data.len());
    }
    self_.length = data.len() as _;
    self_.timeStamp = timestamp;
    self_
}

#[inline]
#[allow(non_snake_case)]
fn CreateMIDIPacketList(timestamp: u64, data: &[u8]) -> coremidi_sys::MIDIPacketList {
    let packet = CreateMIDIPacket(timestamp, data);
    coremidi_sys::MIDIPacketList {
        numPackets: 1,
        packet: [packet],
    }
}

impl MIDISender {
    pub(crate) fn new(
        client: &MIDIClient,
        endpoint: MIDIEndpoint,
        port: coremidi_sys::MIDIPortRef,
    ) -> Self {
        // Self { inner }
        // todo!()
        Self { endpoint, port }
    }
    // pub fn n

    //todo: what's that into pattern
    pub fn send(&self, timestamp: impl Into<Option<u64>>, data: &[u8]) {
        let timestamp = timestamp.into().unwrap_or(0);
        let list = CreateMIDIPacketList(timestamp, data);

        unsafe {
            coremidi_sys::MIDISend(self.port, self.endpoint.inner(), &list);
        }
    }

    // pub fn send(&self, t: &MIDIPacket) -> Result<(), std::sync::mpsc::SendError<MIDIPacket>> {
    //     let z = coremidi_sys::MIDIPacket {
    //         length: 0,
    //         timeStamp: 0,
    //         data: [0; 256],
    //         __padding: [0; 2],
    //     };
    //     unsafe {
    //         coremidi_sys::MIDISend(self.port, self.endpoint.inner(), std::ptr::null());
    //     }
    //     todo!()
    // }
}
