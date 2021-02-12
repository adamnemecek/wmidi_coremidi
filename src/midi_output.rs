use crate::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct MIDIOutput {
    inner: MIDIOutputImpl,
    // hash: u64,
}

// impl PartialEq for MIDIOutput {
//     fn eq(&self, other: &Self) -> bool {
//         self.hash == other.hash
//     }
// }

// impl Eq for MIDIOutput {}

impl MIDIOutput {
    pub(crate) fn new(endpoint: MIDIEndpoint) -> Self {
        let inner = MIDIOutputImpl::new(endpoint);
        // let hash = crate::hash(&port);
        Self {
            // inner: std::sync::Arc::new(std::sync::Mutex::new(port)),
            inner, // hash,
        }
    }
}

impl MIDIPort for MIDIOutput {
    fn id(&self) -> MIDIPortID {
        // MIDIPortID::new(self.inner.borrow().id())
        self.inner.id()
    }


    fn open(&self) {
        self.inner.open()
    }
}

impl MIDIOutput {
    // pub fn sender(&self) -> MIDISender {
    //     self.inner.sender()
    // }

    // pub fn display_name1(&self) -> String {
    //     self.inner.display_name1()
    // }

    pub fn display_name(&self) -> &str {
        self.inner.display_name()
    }

    pub fn manufacturer(&self) -> &str {
        self.inner.manufacturer()
    }

    // pub fn name(&self) -> &str {
    //     self.inner.name()
    // }

    // fn open(&mut self) {
    //     self.inner.open();
    // }

    // fn close(&mut self) {
    //     self.inner.close();
    // }

    // fn connection(&self) -> MIDIPortConnectionState {
    //     self.inner.connection()
    // }
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

#[derive(Clone, PartialEq, Eq)]
struct MIDIOutputImpl {
    endpoint: MIDIEndpoint,
    // port: Option<coremidi_sys::MIDIPortRef>,
    // client: MIDIClient,
}

impl MIDIOutputImpl {
    fn new(endpoint: MIDIEndpoint) -> Self {
        Self {
            // client,
            endpoint,
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

impl MIDIOutputImpl {
    fn open(&self) {
        // self.inner.open()
    }

    fn id(&self) -> MIDIPortID {
        self.endpoint.id()
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

    // fn connection(&self) -> MIDIPortConnectionState {
    //     if self.port.is_some() {
    //         MIDIPortConnectionState::Open
    //     } else {
    //         MIDIPortConnectionState::Closed
    //     }
    // }

    // fn sender(&self) -> MIDISender {
    //     let port = self.client.create_output_port("");
    //     MIDISender::new(&self.client, self.endpoint.clone(), port)
    // }

    // fn open(&mut self) {
    //     if self.connection() == MIDIPortConnectionState::Open {
    //         return;
    //     }
    //     self.port = Some(self.client.create_output_port(""));
    // }

    // fn close(&mut self) {
    //     if self.connection() == MIDIPortConnectionState::Closed {
    //         return;
    //     }
    //     self.endpoint.flush();
    //     self.port = None;
    // }
}

impl std::fmt::Display for MIDIOutputImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIDIOutput")
    }
}

impl std::fmt::Debug for MIDIOutputImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIDIOutput")
    }
}
