use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIAccess {
    // inner: std::rc::Rc<MIDIAccessImpl>,
    inner: std::sync::Arc<std::sync::Mutex<MIDIAccessImpl>>,
}

impl MIDIAccess {
    pub fn new(name: &str) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        let inner = std::sync::Arc::new(std::sync::Mutex::new(MIDIAccessImpl::new(name, tx)));
        // let clone = inner.clone();
        // std::thread::spawn(move || {
        //     let d = rx.recv().unwrap();
        //     clone.lock().unwrap().notification(d);
        // });
        Self { inner }
    }

    pub fn inputs(&self) -> MIDIPortMap<MIDIInput> {
        self.inner.lock().unwrap().inputs()
        // todo!()
    }

    pub fn outputs(&self) -> MIDIPortMap<MIDIOutput> {
        self.inner.lock().unwrap().outputs()
        // todo!()
    }
}

struct MIDIAccessImpl {
    client: MIDIClient,
    inputs: MIDIPortMap<MIDIInput>,
    outputs: MIDIPortMap<MIDIOutput>,
}

impl MIDIAccessImpl {
    fn notification(&mut self, u: u32) {}
    pub fn new(name: &str, tx: std::sync::mpsc::Sender<u32>) -> Self {
        let client = MIDIClient::new(name, tx);
        let inputs = MIDIPortMap::<MIDIInput>::new(&client);
        let outputs = MIDIPortMap::<MIDIOutput>::new(&client);

        Self {
            client,
            inputs,
            outputs,
        }
    }

    pub fn inputs(&self) -> MIDIPortMap<MIDIInput> {
        self.inputs.clone()
    }

    pub fn outputs(&self) -> MIDIPortMap<MIDIOutput> {
        self.outputs.clone()
    }
}
