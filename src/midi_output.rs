use crate::prelude::*;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MIDIOutput {
    inner: std::rc::Rc<MIDIOutputImpl>
}

impl MIDIOutput {
    fn open(&mut self) {
        self.inner.open();
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIOutputImpl {
    endpoint: MIDIEndpoint,
    port: Option<coremidi_sys::MIDIPortRef>,
    client: MIDIClient,
}

impl std::hash::Hash for MIDIOutputImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.endpoint.hash(state);
    }
}

impl MIDIOutputImpl {
    fn open(&mut self) {
        self.port = Some(self.client.create_output_port(""));
    }

    pub fn connection(&self) -> MIDIPortConnectionState {
        if self.port.is_some() {
            MIDIPortConnectionState::Open
        } else {
            MIDIPortConnectionState::Closed
        }
    }

    fn close(&mut self) {
        if self.connection() == MIDIPortConnectionState::Closed {
            return;
        }
        self.endpoint.flush();
        self.port = None;
    }
}
