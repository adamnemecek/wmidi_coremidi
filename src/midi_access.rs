use crate::{
    prelude::*,
    MIDIOutput,
};

pub struct MIDIAccess {
    // inner: std::rc::Rc<MIDIAccessImpl>,
    inner: std::sync::Arc<std::sync::Mutex<MIDIAccessImpl>>,
}

impl MIDIAccess {
    pub fn new(name: &str, f: impl Fn(MIDINotification) -> () + 'static) -> Self {
        // let access = MIDIAccessImpl::new(name, f);
        // todo!()
        Self {
            inner: std::sync::Arc::new(MIDIAccessImpl::new(name, f).into()),
        }
        // let (tx, rx) = std::sync::mpsc::channel();

        // let inner = std::sync::Arc::new(std::sync::Mutex::new(MIDIAccessImpl::new(name, tx)));
        // let clone = inner.clone();
        // std::thread::spawn(move || {
        //     let d = rx.recv().unwrap();
        //     clone.lock().unwrap().notification(d);
        // });
        // Self { inner }
        // todo!()
    }

    pub fn inputs(&self) -> MIDIPortMap<MIDIInput> {
        self.inner.lock().unwrap().inputs()
    }

    pub fn outputs(&self) -> MIDIPortMap<MIDIOutput> {
        self.inner.lock().unwrap().outputs()
    }
}

struct MIDIAccessImpl {
    client: MIDIClient,
    inputs: MIDIPortMap<MIDIInput>,
    outputs: MIDIPortMap<MIDIOutput>,
}

impl MIDIAccessImpl {
    fn notification(&mut self, u: u32) {}

    pub fn new(name: &str, f: impl Fn(MIDINotification) -> () + 'static) -> Self {
        let client = MIDIClient::new(name, f);
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
