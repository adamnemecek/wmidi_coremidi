use crate::prelude::*;

// typedef void (^MIDIReadBlock)(const MIDIPacketList *pktlist, void *srcConnRefCon);

type MIDIReadBlock = block::Block<(*const coremidi_sys::MIDIPacketList, std::ffi::c_void), ()>;

#[derive(Clone, PartialEq, Eq)]
pub struct MIDIInput {
    inner: std::rc::Rc<std::cell::RefCell<MIDIInputImpl>>,
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIInputImpl {
    endpoint: MIDIEndpoint,
}

impl MIDIInputImpl {}
