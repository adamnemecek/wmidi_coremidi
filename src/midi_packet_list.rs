// use coremidi_sys::MIDIPacket;

pub struct MIDIPacketListIterator<'a> {
    count: usize,
    inner: *const coremidi_sys::MIDIPacket,
    ph: std::marker::PhantomData<&'a ()>,
}

impl<'a> MIDIPacketListIterator<'a> {}

impl<'a> Iterator for MIDIPacketListIterator<'a> {
    type Item = &'a coremidi_sys::MIDIPacket;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            // let packet = unsafe { &*(self.inner as *const coremidi_sys::MIDIPacket) };
            let packet = unsafe { self.inner.as_ref().unwrap() };
            self.count -= 1;
            self.inner = unsafe { coremidi_sys::MIDIPacketNext(self.inner) };
            Some(packet)
        } else {
            None
        }
    }
}

impl<'a> MIDIPacketListIterator<'a> {
    pub(crate) fn new(list: &'a coremidi_sys::MIDIPacketList) -> Self {
        // coremidi_sys::MIDIPacketNext(pkt)
        todo!()
    }
}
