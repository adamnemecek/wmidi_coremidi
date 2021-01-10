use crate::prelude::*;

pub struct MIDIOutput {
    endpoint: MIDIEndpoint,
    port: Option<coremidi_sys::MIDIPortRef>,
    client: MIDIClient,
}

impl MIDIOutput {
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
