use crate::prelude::*;

#[derive(Clone, PartialEq, Eq)]
struct MIDIClient {
    inner: coremidi_sys::MIDIClientRef
}

impl MIDIClient {
    pub fn new(name: &str) -> Self {
        Self { inner: MIDIClientCreate(name) }
    }
}

impl Drop for MIDIClient {
    fn drop(&mut self) {
        unsafe {
            os_assert(coremidi_sys::MIDIClientDispose(self.inner));
        }
    }
}

fn MIDIClientCreate(name: &str) -> coremidi_sys::MIDIClientRef {
    // let mut client_ref = MaybeUninit::uninit();
    // coremidi_sys::MIDIClientCreateWithBlock(name, outClient, notifyBlock)
    // coremidi_sys::MIDIClientCreateWithBlock()
    // inner
    todo!()
}