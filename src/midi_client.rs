use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct MIDIClient {
    inner: std::rc::Rc<MIDIClientImpl>,
}

impl MIDIClient {
    pub fn new(name: &str) -> Self {
        Self {
            inner: std::rc::Rc::new(MIDIClientImpl::new(name)),
        }
    }

    pub fn create_input_port(
        &self,
        name: &str,
        f: impl FnMut(&coremidi_sys::MIDIPacketList),
    ) -> coremidi_sys::MIDIPortRef {
        self.inner.create_input_port(name, f)
    }

    pub fn create_output_port(&self, name: &str) -> coremidi_sys::MIDIPortRef {
        self.inner.create_output_port(name)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIClientImpl {
    inner: coremidi_sys::MIDIClientRef,
}

type MIDIReceiveBlock = block::Block<(*const coremidi_sys::MIDIPacketList), ()>;

// coremidi_sys::MIDIReadBlock
impl MIDIClientImpl {
    fn new(name: &str) -> Self {
        Self {
            inner: MIDIClientCreate(name),
        }
    }

    fn new_with_notification(name: &str) -> Self {
        todo!()
    }

    fn create_input_port(
        &self,
        name: &str,
        f: impl FnMut(&coremidi_sys::MIDIPacketList),
    ) -> coremidi_sys::MIDIPortRef {
        // let notify_block = block::ConcreteBlock::new(move |evt: &SharedEventRef, val: u64| {
        // let b = block::ConcreteBlock::<(), ()>::new();
        todo!()
    }

    fn create_output_port(&self, name: &str) -> coremidi_sys::MIDIPortRef {
        let mut out = 0;
        let name = core_foundation::string::CFString::new(name);
        unsafe {
            // os_assert(coremidi_sys::MIDIOutputPortCreate(
            //     self.inner, name.as_mut_ptr(), &mut out,
            // ));
            todo!();
        }
        out
    }
}

impl std::hash::Hash for MIDIClientImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl PartialOrd for MIDIClientImpl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
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
    // let mut client_ref = std::mem::MaybeUninit::uninit();
    // coremidi_sys::MIDIClientCreateWithBlock(name, outClient, notifyBlock)
    // coremidi_sys::MIDIClientCreateWithBlock()
    // coremidi_sys::MIDIClientCreateWithBlock(name, outClient, notifyBlock)
    // inner
    // *client_ref
    todo!()
}
