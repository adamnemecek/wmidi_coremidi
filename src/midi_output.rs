// use coremidi_sys::{
//     // MIDIOutputPortCreate,
//     MIDIPacketList,
//     MIDIPortRef,
//     MIDIReceived,
// };

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct MIDIOutput {
    inner: MIDIOutputImpl,
}

impl MIDIOutput {
    pub(crate) fn new(client: MIDIClient, endpoint: MIDIEndpoint) -> Self {
        let inner = MIDIOutputImpl::new(client, endpoint);
        Self {
            inner,
        }
    }
}

impl MIDIPort for MIDIOutput {
    fn id(&self) -> MIDIPortID {
        self.inner.id()
    }

    fn kind(&self) -> MIDIPortKind {
        MIDIPortKind::Output
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn display_name(&self) -> &str {
        self.inner.display_name()
    }

    fn manufacturer(&self) -> &str {
        self.inner.manufacturer()
    }

    fn open(&mut self) {
        self.inner.open()
    }

    fn close(&mut self) {
        self.inner.close()
    }

    fn connection(&self) -> MIDIPortConnectionState {
        self.inner.connection()
    }
}

impl MIDIOutput {
    // pub fn sender(&self) -> MIDISender {
    //     self.inner.sender()
    // }

    // pub fn name(&self) -> &str {
    //     self.inner.name()
    // }

    // fn close(&mut self) {
    //     self.inner.close();
    // }

    // fn connection(&self) -> MIDIPortConnectionState {
    //     self.inner.connection()
    // }

    pub fn send(&mut self, data: &[u8], offset: impl Into<Option<std::time::Duration>>) {
        self.inner.send(data, offset);
    }
}

impl std::fmt::Display for MIDIOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for MIDIOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::hash::Hash for MIDIOutput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

#[derive(Clone)]
struct MIDIOutputImpl {
    endpoint: MIDIEndpoint,
    port_ref: Option<coremidi_sys::MIDIPortRef>,
    on_state_change: Option<std::rc::Rc<dyn Fn(MIDIOutput) -> ()>>,
    client: MIDIClient,
}

impl PartialEq for MIDIOutputImpl {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint
    }
}

impl Eq for MIDIOutputImpl {}

impl MIDIOutputImpl {
    fn new(client: MIDIClient, endpoint: MIDIEndpoint) -> Self {
        Self {
            client,
            endpoint,
            port_ref: None,
            on_state_change: None,
            // port: None,
        }
    }
}

impl PartialOrd for MIDIOutputImpl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.endpoint.partial_cmp(&other.endpoint)
    }
}

impl Ord for MIDIOutputImpl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.endpoint.cmp(&other.endpoint)
    }
}

impl std::hash::Hash for MIDIOutputImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.endpoint.hash(state);
    }
}

// @inline(__always) fileprivate
// func MIDIOutputPortCreate(ref: MIDIClientRef) -> MIDIPortRef {
//     var port = MIDIPortRef()
//     OSAssert(MIDIOutputPortCreate(ref, "MIDI output" as CFString, &port))
//     return port
// }

fn midi_output_port_create(
    client_ref: coremidi_sys::MIDIClientRef,
    name: &str,
) -> coremidi_sys::MIDIPortRef {
    use core_foundation::base::TCFType;
    let mut out = 0;
    let name = core_foundation::string::CFString::new(name);
    unsafe {
        os_assert(coremidi_sys::MIDIOutputPortCreate(
            client_ref,
            name.as_concrete_TypeRef(),
            &mut out,
        ));
    }
    out
}

impl MIDIOutputImpl {
    fn connection(&self) -> MIDIPortConnectionState {
        if self.port_ref.is_none() {
            MIDIPortConnectionState::Closed
        } else {
            MIDIPortConnectionState::Open
        }
    }

    fn open(&mut self) {
        self.port_ref = Some(midi_output_port_create(self.client.inner(), "MIDI Output"));
        // self.inner.open()
    }

    fn id(&self) -> MIDIPortID {
        self.endpoint.id()
    }

    fn name(&self) -> &str {
        self.endpoint.name()
    }

    fn display_name(&self) -> &str {
        self.endpoint.display_name()
    }

    // fn display_name1(&self) -> String {
    //     self.endpoint.display_name1()
    // }

    fn manufacturer(&self) -> &str {
        self.endpoint.manufacturer()
    }
    // pub fn name(&self) -> &str {
    // self.endpoint.name()
    // }

    // public func send<S: Sequence>(_ data: S, offset: Double? = nil) -> MIDIOutput where S.Iterator.Element == UInt8 {
    //     open()
    //     var lst = MIDIPacketList(data)
    //     lst.send(to: self, offset: offset)

    //     return self
    // }

    // public func clear() {
    //     endpoint.flush()
    // }
    pub fn send(&mut self, data: &[u8], offset: impl Into<Option<std::time::Duration>>) {
        self.open();
        // _ = offset.map {
        //     // NOTE: AudioGetCurrentHostTime() CoreAudio method is only available on macOS
        //     let current = AudioGetCurrentHostTime()
        //     let _offset = AudioConvertNanosToHostTime(UInt64($0 * 1000000))

        //     let ts = current + _offset
        //     packet.timeStamp = ts
        // }
        // let timestamp =
        // let d: std::time::Duration = offset.into();
        // let timestamp = offset.into();
        // d

        // OSAssert(MIDISend(output.ref, output.endpoint.ref, &self))
        // /// this let's us propagate the events to everyone subscribed to this
        // /// endpoint not just this port, i'm not sure if we actually want this
        // /// but for now, it let's us create multiple ports from different MIDIAccess
        // /// objects and have them all receive the same messages
        // OSAssert(MIDIReceived(output.endpoint.ref, &self))
        let mut d = [0; 256];
        unsafe {
            data.as_ptr()
                .copy_to_nonoverlapping(d.as_mut_ptr(), data.len());
        }

        let packet = coremidi_sys::MIDIPacket {
            timeStamp: 0,
            length: data.len() as _,
            data: d,
            __padding: [0; 2],
        };
        let packet_list = coremidi_sys::MIDIPacketList {
            numPackets: 1,
            packet: [packet; 1],
        };
        unsafe {
            os_assert(coremidi_sys::MIDISend(
                self.port_ref.unwrap(),
                self.endpoint.inner(),
                &packet_list,
            ));

            os_assert(coremidi_sys::MIDIReceived(
                self.endpoint.inner(),
                &packet_list,
            ));
        };

        // let port = self.client.create_output_port("");
        // MIDISender::new(&self.client, self.endpoint.clone(), port)
    }

    pub fn clear(&self) {
        self.endpoint.flush();
    }

    // fn open(&mut self) {
    //     if self.connection() == MIDIPortConnectionState::Open {
    //         return;
    //     }
    //     self.port = Some(self.client.create_output_port(""));
    // }

    fn close(&mut self) {
        if self.connection() == MIDIPortConnectionState::Closed {
            return;
        }
        self.endpoint.flush();
        self.port_ref = None;
        if let Some(ref on_state_change) = self.on_state_change {
            on_state_change(MIDIOutput::new(self.client.clone(), self.endpoint.clone()))
        }
        self.on_state_change = None;
    }
}

impl std::fmt::Display for MIDIOutputImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// let type: String
// if self.type == .input {
//     type = "MIDIInput"
// } else {
//     type = "MIDIOutput"
// }
// return "\(type) \(name) by \(manufacturer), connection: \(connection) (id: \(id))"

impl std::fmt::Debug for MIDIOutputImpl {
    // todo: why does it crash on manufacturer?
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MIDIOutput {{ name {:?} by {:?} connection: {:?}, id: {:?} }}",
            self.display_name(),
            self.manufacturer(),
            self.connection(),
            self.id()
        )
    }
}
