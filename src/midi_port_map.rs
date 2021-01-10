use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::collections::HashMap<u32, T>,
}

impl<T: MIDIPort> MIDIPortMap<T> {}

impl MIDIPortMap<MIDIOutput> {
    pub(crate) fn new(client: MIDIClient) -> Self {
        let count = unsafe { coremidi_sys::MIDIGetNumberOfDestinations() } as _;
        let mut inner = std::collections::HashMap::with_capacity(count);

        unsafe {
            for i in 0..count {
                let endpoint = coremidi_sys::MIDIGetSource(i as _);
                let output = MIDIOutput::new(client.clone(), MIDIEndpoint::new(endpoint));
                inner.insert(output.id(), output);
            }
        }
        Self { inner }
    }
}
