use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIPortMapImplIterator<'a, T: MIDIPort> {
    inner: std::collections::hash_map::Iter<'a, MIDIPortID, T>,
}

impl<'a, T: MIDIPort> Iterator for MIDIPortMapImplIterator<'a, T> {
    type Item = (&'a MIDIPortID, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Clone)]
pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::sync::Arc<MIDIPortMapImpl<T>>,
}

impl<T: MIDIPort> MIDIPortMap<T> {
    pub fn iter(&self) -> MIDIPortMapImplIterator<'_, T> {
        self.inner.iter()
    }
}
impl MIDIPortMap<MIDIInput> {
    pub(crate) fn new(client: &MIDIClient) -> Self {
        let a = MIDIPortMapImpl::<MIDIInput>::new();
        let inner = std::sync::Arc::new(a);
        Self { inner }
    }
}

impl MIDIPortMap<MIDIOutput> {
    pub(crate) fn new(client: &MIDIClient) -> Self {
        let a = MIDIPortMapImpl::<MIDIOutput>::new(client);
        let inner = std::sync::Arc::new(a);
        Self { inner }
    }
}

impl<T: MIDIPort> std::ops::Index<MIDIPortID> for MIDIPortMap<T> {
    type Output = T;
    fn index(&self, index: MIDIPortID) -> &T {
        &self.inner[index]
    }
}

// impl<T: MIDIPort> std::ops::IndexMut<MIDIPortID> for MIDIPortMap<T> {
//     fn index_mut(&mut self, index: MIDIPortID) -> &mut T {
//         self.inner.get_mut(&index).unwrap()
//     }
// }

pub struct MIDIPortMapImpl<T: MIDIPort> {
    inner: std::collections::HashMap<MIDIPortID, T>,
}

impl<T: MIDIPort> MIDIPortMapImpl<T> {
    pub(crate) fn insert(&mut self, port: T) {
        debug_assert!(!self.inner.contains_key(&port.id()));

        self.inner.insert(port.id(), port);
    }

    pub fn iter(&self) -> MIDIPortMapImplIterator<'_, T> {
        MIDIPortMapImplIterator {
            inner: self.inner.iter(),
        }
    }
}

impl<T: MIDIPort> std::ops::Index<MIDIPortID> for MIDIPortMapImpl<T> {
    type Output = T;
    fn index(&self, index: MIDIPortID) -> &T {
        &self.inner[&index]
    }
}

impl<T: MIDIPort> std::ops::IndexMut<MIDIPortID> for MIDIPortMapImpl<T> {
    fn index_mut(&mut self, index: MIDIPortID) -> &mut T {
        self.inner.get_mut(&index).unwrap()
    }
}

impl MIDIPortMapImpl<MIDIInput> {
    pub(crate) fn new() -> Self {
        let count = unsafe { coremidi_sys::MIDIGetNumberOfSources() } as _;
        let mut inner = std::collections::HashMap::with_capacity(count);

        unsafe {
            for i in 0..count {
                let endpoint = coremidi_sys::MIDIGetSource(i as _);
                assert!(endpoint != 0);
                let output = MIDIInput::new( MIDIEndpoint::new(endpoint));
                inner.insert(output.id(), output);
            }
        }
        Self { inner }
    }
}

impl MIDIPortMapImpl<MIDIOutput> {
    pub(crate) fn new(client: &MIDIClient) -> Self {
        let count = unsafe { coremidi_sys::MIDIGetNumberOfDestinations() } as _;
        let mut inner = std::collections::HashMap::with_capacity(count);

        unsafe {
            for i in 0..count {
                let endpoint = coremidi_sys::MIDIGetDestination(i as _);
                assert!(endpoint != 0);
                let output = MIDIOutput::new(client.clone(), MIDIEndpoint::new(endpoint));
                inner.insert(output.id(), output);
            }
        }
        Self { inner }
    }

    // pub(crate) fn new1(client: MIDIClient) -> Self {
    //     let mut inner = std::collections::HashMap::new();
    //     let mut i = MIDISourceIterator::new();
    //     for endpoint in i {
    //         let output = MIDIOutput::new(client.clone(), endpoint);
    //         inner.insert(output.id(), output);
    //     }
    //     Self { inner }
    // }
}

pub struct MIDISourceIterator {
    len: usize,
    index: usize,
}

impl MIDISourceIterator {
    pub fn new() -> Self {
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
            let index = self.index;
            self.index += 1;
            Some(MIDIEndpoint::new(unsafe {
                coremidi_sys::MIDIGetSource(index as _)
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
            let index = self.index;
            self.index += 1;
            Some(MIDIEndpoint::new(unsafe {
                coremidi_sys::MIDIGetDestination(index as _)
            }))
        }
    }
}
