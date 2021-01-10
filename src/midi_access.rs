use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIAccess {
    inner: std::rc::Rc<MIDIAccessImpl>,
}

struct MIDIAccessImpl {
    client: MIDIClient,
    inputs: MIDIPortMap<MIDIInput>,
    outputs: MIDIPortMap<MIDIOutput>,
}

impl MIDIAccessImpl {
    fn new(name: &str) -> Self {
        let client = MIDIClient::new(name);
        let inputs = MIDIPortMap::<MIDIInput>::new(&client);
        let outputs = MIDIPortMap::<MIDIOutput>::new(&client);

        Self {
            client,
            inputs,
            outputs,
        }
    }
}
