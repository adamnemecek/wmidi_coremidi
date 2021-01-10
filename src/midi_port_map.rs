use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::collections::HashMap<u32, T>,
}

impl<T: MIDIPort> MIDIPortMap<T> {}

impl MIDIPortMap<MIDIInput> {
    pub(crate) fn new(client: MIDIClient) -> Self {
        let count = unsafe { coremidi_sys::MIDIGetNumberOfSources() } as _;
        let mut inner = std::collections::HashMap::with_capacity(count);

        unsafe {
            for i in 0..count {
                let endpoint = coremidi_sys::MIDIGetDestination(i as _);
                let output = MIDIInput::new(client.clone(), MIDIEndpoint::new(endpoint));
                inner.insert(output.id(), output);
            }
        }
        Self { inner }
    }
}

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

    pub(crate) fn new1(client: MIDIClient) -> Self {
        let mut inner = std::collections::HashMap::new();
        let mut i = MIDISourceIterator::new();
        while let Some(endpoint) = i.next() {
            let output = MIDIOutput::new(client.clone(), endpoint);
            inner.insert(output.id(), output);
        }
        Self { inner }
    }
}

struct MIDISourceIterator {
    len: usize,
    index: usize,
}

impl MIDISourceIterator {
    fn new() -> Self {
        Self {
            index: 0,
            len: unsafe { coremidi_sys::MIDIGetNumberOfSources() } as _,
        }
    }
}

impl Iterator for MIDISourceIterator {
    type Item = MIDIEndpoint;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            Some(MIDIEndpoint::new(unsafe {
                coremidi_sys::MIDIGetSource(self.index as _)
            }))
        }
    }
}

struct MIDIDestinationIterator {
    len: usize,
    index: usize,
}

impl MIDIDestinationIterator {
    fn new() -> Self {
        Self {
            index: 0,
            len: unsafe { coremidi_sys::MIDIGetNumberOfDestinations() } as _,
        }
    }
}

impl Iterator for MIDIDestinationIterator {
    type Item = MIDIEndpoint;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            Some(MIDIEndpoint::new(unsafe {
                coremidi_sys::MIDIGetDestination(self.index as _)
            }))
        }
    }
}
