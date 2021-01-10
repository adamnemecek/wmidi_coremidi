use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MIDIClient {
    inner: std::rc::Rc<MIDIClientImpl>,
}

impl MIDIClient {
    pub fn new(name: &str) -> Self {
        Self {
            inner: std::rc::Rc::new(MIDIClientImpl::new(name)),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIClientImpl {
    inner: coremidi_sys::MIDIClientRef,
}

impl std::hash::Hash for MIDIClientImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl MIDIClientImpl {
    fn new(name: &str) -> Self {
        Self {
            inner: MIDIClientCreate(name),
        }
    }
}

impl Drop for MIDIClientImpl {
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
