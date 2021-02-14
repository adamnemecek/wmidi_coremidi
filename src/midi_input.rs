use coremidi_sys::{
    MIDIInputPortCreate,
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

    pub fn set_on_midi_message(&mut self, f: MIDIInputFn) {
        self.inner.set_on_midi_message(f)
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

    fn display_name(&self) -> &str {
        self.inner.display_name()
    }

    fn manufacturer(&self) -> &str {
        self.inner.manufacturer()
    }

    fn close(&self) {
        todo!()
    }

    fn open(&mut self) {}

    fn connection(&self) -> MIDIPortConnectionState {
        self.inner.connection()
    }
}

pub type MIDIInputFn = std::rc::Rc<dyn for<'r, 's> Fn(&'r MIDIEvent<'s>) -> ()>;
// pub type MIDIChangeFn = std::rc::Rc<dyn

#[derive(Clone)]
struct MIDIInputImpl {
    client: MIDIClient,
    endpoint: MIDIEndpoint,
    port_ref: coremidi_sys::MIDIPortRef,
    input_fn: Option<MIDIInputFn>,
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
            input_fn: None,
            port_ref: 0,
        }
    }

    fn display_name(&self) -> &str {
        self.endpoint.display_name()
    }

    fn manufacturer(&self) -> &str {
        self.endpoint.manufacturer()
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
        if self.connection() == MIDIPortConnectionState::Open {
            return;
        }
        // let input_fn = self.input_fn.clone();
        self.port_ref = midi_input_port_create(self.client.inner(), "", move |event| {
            //
            // if let Some(ref input_fn) = input_fn {
            //     input_fn(event);
            // }
        })
        .unwrap();
        println!("opened");

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

    fn set_on_midi_message(&mut self, input_fn: MIDIInputFn) {
        self.close();
        self.input_fn = Some(input_fn);
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
        write!(
            f,
            "MIDIInput {{ name {:?} by {:?} connection: {:?}, id: {:?} }}",
            self.display_name(),
            self.manufacturer(),
            self.connection(),
            self.id()
        )
    }
}

impl std::hash::Hash for MIDIInputImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.endpoint.id().hash(state)
    }
}
// #[repr(transparent)]
// struct Block {

// }

// pub type MIDIReadBlock = block::RcBlock<(*const coremidi_sys::MIDIPacketList, *mut std::ffi::c_void, ), ()>;
pub type MIDIReadBlock = block::Block<(*const coremidi_sys::MIDIPacketList, *mut std::ffi::c_void), ()>;

#[link(name = "CoreMIDI", kind = "framework")]
extern "C" {
    fn MIDIInputPortCreateWithBlock(
        client: u32,
        portName: *const core_foundation::string::__CFString,
        outPort: *mut u32,
        // readBlock: &block::Block<(*const std::ffi::c_void, *mut std::ffi::c_void), ()>,
        // readBlock: *const std::ffi::c_void,
        readBlock: &MIDIReadBlock,
    ) -> i32;
}

fn midi_input_port_create(
    client: u32,
    name: &str,
    f: impl Fn(&MIDIEvent) -> () + 'static,
) -> Option<u32> {
    use core_foundation::base::TCFType;

    let block = block::ConcreteBlock::new(
        move |packet: *const coremidi_sys::MIDIPacketList, _: *mut std::ffi::c_void| {
            todo!("callback");
            // let packet = unsafe { packet.as_ref().unwrap() };
            // let mut i = MIDIPacketListIterator::new(packet);
            // while let Some(ref next) = i.next() {
            //     f(next);
            // }
        },
    )
    .copy();

    // let block = & *cblock.copy();
    // unsafe { CFRunLoopPerformBlock(run_loop_ref as *mut c_void, kCFRunLoopDefaultMode, block); }

    let name = core_foundation::string::CFString::new(name);
    let mut out = 0;
    // let block_ref = &mut block;
    // let p: std::ffi::c_void = unsafe { std::mem::transmute(cblock) };
    unsafe {
        os_assert(MIDIInputPortCreateWithBlock(
            client,
            name.as_concrete_TypeRef(),
            &mut out,
            // &mut block.as_ptr(),
            // block_ref as coremidi_sys::MIDIReadBlock,
            // block_ref as *const _,
            // block_ref as *mut std::ffi::c_void,
            // &cblock.copy(),
            // std::mem::transmute(cblock)
            // &*block as *const block::Block<_, _> as *const std::ffi::c_void,
            &block,
        ));
    }
    assert!(out != 0);
    // unsafe {
    //     coremidi_sys::MIDIInputPortCreate(client, portName, readProc, refCon, outPort);
    // }

    // if err == 0 {
    Some(out)
    // } else {
    // None
    // }
}
