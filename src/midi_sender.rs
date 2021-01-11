// similar to std::sync::mpsc::Receiver
use crate::prelude::*;
use crate::MIDIPacket;

pub struct MIDISender {
    // inner: std::sync::mpsc::Sender<MIDIPacket>,
    endpoint: MIDIEndpoint,
    port: coremidi_sys::MIDIPortRef,
}

unsafe impl Send for MIDISender {}
impl !Sync for MIDISender {}

impl MIDISender {
    pub(crate) fn new(
        client: &MIDIClient,
        endpoint: MIDIEndpoint,
        port: coremidi_sys::MIDIPortRef,
    ) -> Self {
        // Self { inner }
        todo!()
    }
    // pub fn n

    pub fn send(&self, t: &MIDIPacket) -> Result<(), std::sync::mpsc::SendError<MIDIPacket>> {
        unsafe {
            coremidi_sys::MIDISend(self.port, self.endpoint.inner(), std::ptr::null());
        }
        todo!()
    }
}
