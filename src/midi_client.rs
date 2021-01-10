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
        f: impl Fn(&coremidi_sys::MIDIPacketList) + 'static,
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
            inner: MIDIClientCreate(name, |x| { }),
        }
    }

    fn new_with_notification(name: &str) -> Self {
        todo!()
    }

    fn create_input_port(
        &self,
        name: &str,
        f: impl Fn(&coremidi_sys::MIDIPacketList) + 'static,
    ) -> coremidi_sys::MIDIPortRef {
        let mut out = 0;
        let block = block::ConcreteBlock::new(move |p: &coremidi_sys::MIDIPacketList| f(p)).copy();

        unsafe {
            use core_foundation::base::TCFType;

            let name = core_foundation::string::CFString::new(name);
            // let name = core_foundation::string::__CFString::from(&name);
            os_assert(coremidi_sys::MIDIInputPortCreateWithBlock(
                self.inner,
                name.as_concrete_TypeRef(),
                &mut out,
                std::mem::transmute(block),
            ));
        }
        out
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

fn MIDIClientCreate(name: &str, f: impl Fn(MIDINotification) -> ()) -> coremidi_sys::MIDIClientRef {
    let mut out = 0;
    unsafe {
        use core_foundation::base::TCFType;
        let block = block::ConcreteBlock::new(move |notification: u32| {}).copy();
        let name = core_foundation::string::CFString::new(name);
        os_assert(coremidi_sys::MIDIClientCreateWithBlock(
            name.as_concrete_TypeRef(),
            &mut out,
            std::mem::transmute(block),
        ));
    }
    out
}
