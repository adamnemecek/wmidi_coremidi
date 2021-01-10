use crate::prelude::*;

// typedef void (^MIDIReadBlock)(const MIDIPacketList *pktlist, void *srcConnRefCon);

type MIDIReadBlock = block::Block<(*const coremidi_sys::MIDIPacketList, std::ffi::c_void), ()>;

#[derive(Clone, PartialEq, Eq)]
pub struct MIDIInput {
    inner: std::rc::Rc<std::cell::RefCell<MIDIInputImpl>>,
}

impl MIDIInput {
    pub(crate) fn new(client: MIDIClient, endpoint: MIDIEndpoint) -> Self {
        Self {
            inner: std::rc::Rc::new(std::cell::RefCell::new(MIDIInputImpl::new(
                client, endpoint,
            ))),
        }
    }
}

impl std::fmt::Debug for MIDIInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::hash::Hash for MIDIInput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl MIDIPort for MIDIInput {
    fn id(&self) -> MIDIPortID {
        self.inner.borrow().id()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIInputImpl {
    client: MIDIClient,
    endpoint: MIDIEndpoint,
}

impl MIDIInputImpl {
    fn new(client: MIDIClient, endpoint: MIDIEndpoint) -> Self {
        Self { client, endpoint }
    }

    fn id(&self) -> MIDIPortID {
        self.endpoint.id()
    }

    fn open(&self) {
        // let `self` = self as! MIDIInput
        // ref = MIDIInputPortCreate(ref: client.ref) {
        //     `self`.onMIDIMessage?($0)
        // }

        // OSAssert(MIDIPortConnectSource(ref, endpoint.ref, nil))
    }
}

impl std::fmt::Debug for MIDIInputImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIDIClientImpl {}", self.endpoint.display_name())
    }
}

impl std::hash::Hash for MIDIInputImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.endpoint.id().hash(state)
    }
}
