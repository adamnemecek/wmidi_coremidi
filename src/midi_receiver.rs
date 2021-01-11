// similar to std::sync::mpsc::Receiver
use crate::prelude::*;
use crate::MIDIPacket;

pub struct MIDIReceiver {
    endpoint: MIDIEndpoint,
    inner: std::sync::mpsc::Receiver<MIDIPacket>,
}

unsafe impl Send for MIDIReceiver {}
impl !Sync for MIDIReceiver {}

impl MIDIReceiver {
    pub(crate) fn new(
        endpoint: MIDIEndpoint,
        inner: std::sync::mpsc::Receiver<MIDIPacket>,
    ) -> Self {
        Self { inner, endpoint }
    }
    pub fn try_recv(&self) -> Result<MIDIPacket, std::sync::mpsc::TryRecvError> {
        self.inner.try_recv()
    }
}
