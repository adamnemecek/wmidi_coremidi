use crate::prelude::*;

// typedef void (^MIDIReadBlock)(const MIDIPacketList *pktlist, void *srcConnRefCon);

type MIDIReadBlock = block::Block<(*const coremidi_sys::MIDIPacketList, std::ffi::c_void), ()>;

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

    // pub fn set_midi_message_receiver(&mut self, rx: MIDIReceiver) {}
    // pub fn receiver(&self) -> MIDIReceiver {
    //     MIDIReceiver::new()
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
}

#[derive(Clone)]
struct MIDIInputImpl {
    client: MIDIClient,
    endpoint: MIDIEndpoint,
    // receiver: Option<std::sync::mpsc::Receiver<crate::MIDIPacket>>,
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
            // receiver: None,
        }
    }

    fn id(&self) -> MIDIPortID {
        self.endpoint.id()
    }

    pub fn receiver(&self) -> MIDIReceiver {
        // if self.
        let (tx, rx) = std::sync::mpsc::channel();
        // self.client.create_input_port("port", |packet| {
        //     tx.send(packet);
        // });
        let name = format!("MIDIReceiver{}", self.endpoint.display_name());
        self.client.create_input_port(&name, tx);

        MIDIReceiver::new(self.endpoint.clone(), rx)

        // self.receiver = Some(rx);

        // let `self` = self as! MIDIInput
        // ref = MIDIInputPortCreate(ref: client.ref) {
        //     `self`.onMIDIMessage?($0)
        // }

        // OSAssert(MIDIPortConnectSource(ref, endpoint.ref, nil))
    }

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
