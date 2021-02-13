use coremidi_sys::{
    MIDIPortDisconnectSource,
    MIDIPortRef,
};

use crate::prelude::*;

// typedef void (^MIDIReadBlock)(const MIDIPacketList *pktlist, void *srcConnRefCon);

// type MIDIReadBlock = block::Block<(*const coremidi_sys::MIDIPacketList, std::ffi::c_void), ()>;

#[derive(PartialEq, Eq, Clone)]
pub struct MIDIInput {
    inner: MIDIInputImpl,
    // hash: u64,
}

// impl PartialEq for MIDIInput {
//     fn eq(&self, other: &Self) -> bool {
//         self.hash == other.hash
//     }
// }

// impl Eq for MIDIInput {}

impl MIDIInput {
    pub(crate) fn new(client: MIDIClient, endpoint: MIDIEndpoint) -> Self {
        let inner = MIDIInputImpl::new(client, endpoint);
        // let hash = crate::hash(&inner);
        Self {
            // inner: std::sync::Arc::new(std::sync::Mutex::new(inner)),
            inner,
            // hash,
        }
    }

    pub fn set_on_midi_message(&mut self, f: impl Fn(MIDIEvent) -> ()) {
        // close();

        // open();
    }

    // pub fn receiver(&self) -> MIDIReceiver {
    //     self.inner.receiver()
    // }
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
        self.inner.id()
        // todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn open(&self) {}

    fn connection(&self) -> MIDIPortConnectionState {
        self.inner.connection()
    }
}

#[derive(Clone)]
struct MIDIInputImpl {
    client: MIDIClient,
    endpoint: MIDIEndpoint,
    port_ref: coremidi_sys::MIDIPortRef,
    f: Option<std::rc::Rc<dyn Fn(MIDIEvent) -> ()>>,
}
// analogous to

unsafe impl Send for MIDIInputImpl {}

impl PartialEq for MIDIInputImpl {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint
    }
}

impl Eq for MIDIInputImpl {}

impl MIDIInputImpl {
    fn new(client: MIDIClient, endpoint: MIDIEndpoint) -> Self {
        Self {
            client,
            endpoint,
            f: None,
            port_ref: 0,
        }
    }

    fn id(&self) -> MIDIPortID {
        self.endpoint.id()
    }

    fn connection(&self) -> MIDIPortConnectionState {
        if self.port_ref == 0 {
            MIDIPortConnectionState::Closed
        } else {
            MIDIPortConnectionState::Open
        }
    }

    fn open(&mut self) {
        if self.connection() == MIDIPortConnectionState::Closed {
            return;
        }
        MIDIInputPortCreate(self.client.inner(), "", |event| {});

        // MIDIInputPortCreateWithBlock(client, portName, outPort, readBlock)
        // guard connection != .open else { return }

        // switch type {
        // case .input:
        //     let `self` = self as! MIDIInput
        //     ref = MIDIInputPortCreate(ref: client.ref) {
        //         `self`.onMIDIMessage?($0)
        //     }

        //     OSAssert(MIDIPortConnectSource(ref, endpoint.ref, nil))

        // case .output:
        //     ref = MIDIOutputPortCreate(ref: client.ref)
        // }

        // onStateChange?(self)
    }

    fn close(&mut self) {
        // guard connection != .closed else { return }
        if self.connection() == MIDIPortConnectionState::Closed {
            return;
        }
        unsafe {
            coremidi_sys::MIDIPortDisconnectSource(self.port_ref, self.endpoint.inner());
        }

        self.port_ref = 0;

        // switch type {
        // case .input:
        //     OSAssert(MIDIPortDisconnectSource(ref, endpoint.ref))
        // case .output:
        //     endpoint.flush()
        // }

        // ref = 0
        // onStateChange?(self)
        // onStateChange = nil
    }

    fn set_on_midi_message(&mut self, f: impl Into<Option<std::rc::Rc<dyn Fn(MIDIEvent) -> ()>>>) {
        self.close();
        self.f = f.into();
        self.open();
    }

    // pub fn receiver(&self) -> MIDIReceiver {
    //     // if self.
    //     let (tx, rx) = std::sync::mpsc::channel();
    //     // self.client.create_input_port("port", |packet| {
    //     //     tx.send(packet);
    //     // });
    //     let name = format!("MIDIReceiver{}", self.endpoint.display_name());
    //     // self.client.create_input_port(&name, tx);
    //     todo!()

    //     // MIDIReceiver::new(self.endpoint.clone(), rx)

    //     // self.receiver = Some(rx);

    //     // let `self` = self as! MIDIInput
    //     // ref = MIDIInputPortCreate(ref: client.ref) {
    //     //     `self`.onMIDIMessage?($0)
    //     // }

    //     // OSAssert(MIDIPortConnectSource(ref, endpoint.ref, nil))
    // }

    // fn close(&mut self) {
    //     //
    //     self.receiver = None;
    // }
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

#[link(name = "CoreMIDI", kind = "framework")]
extern "C" {
    fn MIDIInputPortCreateWithBlock(
        client: u32,
        portName: *const core_foundation::string::__CFString,
        outPort: *mut u32,
        readBlock: block::RcBlock<(*const coremidi_sys::MIDIPacketList, *mut std::ffi::c_void), ()>,
    ) -> i32;
}

fn MIDIInputPortCreate(client: u32, name: &str, f: impl Fn(&[MIDIPacket])) -> Option<u32> {
    use core_foundation::base::TCFType;
    let mut block = block::ConcreteBlock::new(
        move |packet: *const coremidi_sys::MIDIPacketList, _: *mut std::ffi::c_void| {
            // let i = MIDIPacketListIterator
            todo!("input block");
            // todo!();
            // let i = MIDIPacketListIterator::new(unsafe { packet.as_ref().unwrap() });
            // let p = MIDIPacket::new(0, &[1,2,3]);
            // tx.send(p);
            // println!("here");
            // for e in i {
            //     let packet = MIDIPacket::from(e);
            //     let _ = tx.send(packet);
            // }
        },
    )
    .copy();
    let name = core_foundation::string::CFString::new(name);
    let mut out = 0;
    let err = unsafe {
        MIDIInputPortCreateWithBlock(client, name.as_concrete_TypeRef(), &mut out, block)
    };
    if err == 0 {
        Some(out)
    } else {
        None
    }
}
