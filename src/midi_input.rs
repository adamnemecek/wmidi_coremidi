// use coremidi_sys::{
//     MIDIInputPortCreate,
//     MIDIPortDisconnectSource,
//     MIDIPortRef,
// };

use crate::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct MIDIInput {
    inner: MIDIInputImpl,
}

impl MIDIInput {
    pub(crate) fn new(client: MIDIClient, endpoint: MIDIEndpoint) -> Self {
        let inner = MIDIInputImpl::new(client, endpoint);
        Self { inner }
    }

    pub fn set_on_midi_message(&mut self, f: MIDIInputFn) {
        self.inner.set_on_midi_message(f)
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
        self.inner.id()
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn kind(&self) -> MIDIPortKind {
        MIDIPortKind::Input
    }

    fn display_name(&self) -> &str {
        self.inner.display_name()
    }

    fn manufacturer(&self) -> &str {
        self.inner.manufacturer()
    }

    fn close(&mut self) {
        self.inner.close()
    }

    fn open(&mut self) {
        self.inner.open()
    }

    fn connection(&self) -> MIDIPortConnectionState {
        self.inner.connection()
    }

    fn on_state_change(&self) -> Option<StateChangeFn<Self>> {
        self.inner.on_state_change()
    }

    fn set_on_state_change(&mut self, on_state_change: Option<StateChangeFn<Self>>) {
        self.inner.set_on_state_change(on_state_change);
    }
}

pub type MIDIInputFn = std::rc::Rc<dyn for<'r, 's> Fn(&'r MIDIEvent<'s>) -> ()>;

#[derive(Clone)]
struct MIDIInputImpl {
    client: MIDIClient,
    endpoint: MIDIEndpoint,
    port_ref: Option<coremidi_sys::MIDIPortRef>,
    on_state_change: Option<StateChangeFn<MIDIInput>>,
    input_fn: Option<MIDIInputFn>,
}
// analogous to

// unsafe impl Send for MIDIInputImpl {}

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
            port_ref: None,
            on_state_change: None,
            input_fn: None,
        }
    }

    fn on_state_change(&self) -> Option<StateChangeFn<MIDIInput>> {
        self.on_state_change.clone()
    }

    fn set_on_state_change(&mut self, on_state_change: Option<StateChangeFn<MIDIInput>>) {
        self.on_state_change = on_state_change;
    }

    fn name(&self) -> &str {
        self.endpoint.name()
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
        if self.port_ref.is_none() {
            MIDIPortConnectionState::Closed
        } else {
            MIDIPortConnectionState::Open
        }
    }

    fn open(&mut self) {
        if self.connection() == MIDIPortConnectionState::Open {
            return;
        }
        let input_fn = self.input_fn.clone();
        self.port_ref = midi_input_port_create(self.client.inner(), "", move |event| {
            if let Some(ref input_fn) = input_fn {
                input_fn(event);
            }
        });

        unsafe {
            os_assert(coremidi_sys::MIDIPortConnectSource(
                self.port_ref.unwrap(),
                self.endpoint.inner(),
                std::ptr::null_mut(),
            ));
        }

        // MIDIInputPortCreateWithBlock(client, portName, outPort, readBlock)

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
        if self.connection() == MIDIPortConnectionState::Closed {
            return;
        }
        unsafe {
            os_assert(coremidi_sys::MIDIPortDisconnectSource(
                self.port_ref.unwrap(),
                self.endpoint.inner(),
            ));
        }

        self.port_ref = None;

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

pub type MIDIReadBlock =
    block::Block<(*const coremidi_sys::MIDIPacketList, *mut std::ffi::c_void), ()>;

#[allow(improper_ctypes)]
#[link(name = "CoreMIDI", kind = "framework")]
extern "C" {
    fn MIDIInputPortCreateWithBlock(
        client: u32,
        port_name: *const core_foundation::string::__CFString,
        outPort: *mut u32,
        read_block: &MIDIReadBlock,
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
            let packet = unsafe { packet.as_ref().unwrap() };
            let mut i = MIDIPacketListIterator::new(packet);
            while let Some(ref next) = i.next() {
                f(next);
            }
        },
    )
    .copy();

    let name = core_foundation::string::CFString::new(name);
    let mut out = 0;
    unsafe {
        os_assert(MIDIInputPortCreateWithBlock(
            client,
            name.as_concrete_TypeRef(),
            &mut out,
            &block,
        ));
    }
    assert!(out != 0);

    // if err == 0 {
    Some(out)
    // } else {
    // None
    // }
}
