use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIAccess {
    inner: std::rc::Rc<MIDIAccessImpl>,
}

impl MIDIAccess {
    pub fn new(name: &str) -> Self {
        Self {
            inner: std::rc::Rc::new(MIDIAccessImpl::new(name)),
        }
    }

    pub fn inputs(&self) -> &MIDIPortMap<MIDIInput> {
        self.inner.inputs()
    }

    pub fn outputs(&self) -> &MIDIPortMap<MIDIOutput> {
        self.inner.outputs()
    }
}

struct MIDIAccessImpl {
    client: MIDIClient,
    inputs: MIDIPortMap<MIDIInput>,
    outputs: MIDIPortMap<MIDIOutput>,
}

impl MIDIAccessImpl {
    pub fn new(name: &str) -> Self {
        let client = MIDIClient::new(name);
        let inputs = MIDIPortMap::<MIDIInput>::new(&client);
        let outputs = MIDIPortMap::<MIDIOutput>::new(&client);

        Self {
            client,
            inputs,
            outputs,
        }
    }

    pub fn inputs(&self) -> &MIDIPortMap<MIDIInput> {
        &self.inputs
    }

    pub fn outputs(&self) -> &MIDIPortMap<MIDIOutput> {
        &self.outputs
    }
}
