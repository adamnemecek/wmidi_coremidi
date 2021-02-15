use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIAccess {
    inner: std::rc::Rc<MIDIAccessImpl>,
}

impl MIDIAccess {
    pub fn new(name: &str, f: impl Fn(MIDINotification) -> () + 'static) -> Self {
        Self {
            inner: MIDIAccessImpl::new(name, f).into(),
        }
    }

    pub fn inputs(&self) -> MIDIPortMap<MIDIInput> {
        self.inner.inputs()
    }

    pub fn input_for(&self, output: &MIDIOutput) -> Option<MIDIInput> {
        self.inner.input_for(output)
    }

    pub fn outputs(&self) -> MIDIPortMap<MIDIOutput> {
        self.inner.outputs()
    }

    pub fn output_for(&self, input: &MIDIInput) -> Option<MIDIOutput> {
        self.inner.output_for(input)
    }
}

struct MIDIAccessImpl {
    client: MIDIClient,
    inputs: MIDIPortMap<MIDIInput>,
    outputs: MIDIPortMap<MIDIOutput>,
}

impl MIDIAccessImpl {
    fn notification(&mut self, u: u32) {}

    fn new(name: &str, f: impl Fn(MIDINotification) -> () + 'static) -> Self {
        let client = MIDIClient::new(name, f);
        let inputs = MIDIPortMap::<MIDIInput>::new(&client);
        let outputs = MIDIPortMap::<MIDIOutput>::new(&client);

        Self {
            client,
            inputs,
            outputs,
        }
    }

    fn inputs(&self) -> MIDIPortMap<MIDIInput> {
        self.inputs.clone()
    }

    fn input_for(&self, output: &MIDIOutput) -> Option<MIDIInput> {
        self.inputs
            .iter()
            .find(|(_, input)| input.display_name() == output.display_name())
            .map(|(_, port)| port)
    }

    fn outputs(&self) -> MIDIPortMap<MIDIOutput> {
        self.outputs.clone()
    }

    fn output_for(&self, input: &MIDIInput) -> Option<MIDIOutput> {
        self.outputs
            .iter()
            .find(|(_, output)| output.display_name() == input.display_name())
            .map(|(_, port)| port)
    }
}
