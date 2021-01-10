use crate::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct MIDIOutput {
    inner: std::rc::Rc<std::cell::RefCell<MIDIOutputImpl>>,
}

impl MIDIPort for MIDIOutput {
    fn id(&self) -> u32 {
        self.inner.borrow().id()
    }
}

impl MIDIOutput {
    fn open(&mut self) {
        self.inner.borrow_mut().open();
    }

    fn close(&mut self) {
        self.inner.borrow_mut().close();
    }

    fn connection(&self) -> MIDIPortConnectionState {
        self.inner.borrow().connection()
    }
}

impl std::fmt::Debug for MIDIOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::hash::Hash for MIDIOutput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.borrow().hash(state);
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIOutputImpl {
    endpoint: MIDIEndpoint,
    port: Option<coremidi_sys::MIDIPortRef>,
    client: MIDIClient,
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

impl MIDIOutputImpl {
    fn id(&self) -> u32 {
        todo!()
    }

    pub fn connection(&self) -> MIDIPortConnectionState {
        if self.port.is_some() {
            MIDIPortConnectionState::Open
        } else {
            MIDIPortConnectionState::Closed
        }
    }

    fn open(&mut self) {
        if self.connection() == MIDIPortConnectionState::Open {
            return;
        }
        self.port = Some(self.client.create_output_port(""));
    }

    fn close(&mut self) {
        if self.connection() == MIDIPortConnectionState::Closed {
            return;
        }
        self.endpoint.flush();
        self.port = None;
    }
}

impl std::fmt::Debug for MIDIOutputImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
