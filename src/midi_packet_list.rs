// use coremidi_sys::MIDIPacket;
use crate::prelude::*;

pub struct MIDIPacketListIterator<'a> {
    count: usize,
    inner: *const coremidi_sys::MIDIPacket,
    ph: std::marker::PhantomData<&'a ()>,
}

impl<'a> MIDIPacketListIterator<'a> {
    pub(crate) fn new(list: &'a coremidi_sys::MIDIPacketList) -> Self {
        Self {
            count: list.numPackets as _,
            inner: list.packet.as_ptr(),
            ph: Default::default(),
        }
    }
}

impl<'a> Iterator for MIDIPacketListIterator<'a> {
    type Item = MIDIEvent<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as _, None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            // let packet = unsafe { &*(self.inner as *const coremidi_sys::MIDIPacket) };

            let packet = unsafe { self.inner.as_ref().unwrap() };
            let slice =
                unsafe { std::slice::from_raw_parts(packet.data.as_ptr(), packet.length as _) };
            // MIDIEvent::new(packet.timeStamp, )
            self.count -= 1;
            self.inner = unsafe { coremidi_sys::MIDIPacketNext(self.inner) };
            let event = MIDIEvent::new(packet.timeStamp, slice);
            Some(event)
        } else {
            None
        }
    }
}
