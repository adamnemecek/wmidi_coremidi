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
}
