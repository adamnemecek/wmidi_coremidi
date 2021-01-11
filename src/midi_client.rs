use crate::prelude::*;

// pub trait Hashable {
//     fn get_hash(&self) -> u64;
// }

// impl<T: std::hash::Hash> Hashable for T {
//     fn get_hash(&self) -> u64 {
//         use std::hash::Hasher;
//         use std::collections::hash_map::DefaultHasher;
//         let mut s = DefaultHasher::new();
//         self.hash(&mut s);
//         s.finish()
//     }
// }

pub(crate) fn hash<T: std::hash::Hash>(v: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    let mut s = DefaultHasher::new();
    v.hash(&mut s);
    s.finish()
}

#[derive(Clone)]
pub(crate) struct MIDIClient {
    inner: std::sync::Arc<std::sync::Mutex<MIDIClientImpl>>,
    hash: u64,
}

impl PartialEq for MIDIClient {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for MIDIClient {}
// input_observer: Option<std::sync::Arc<std::sync::Mutex<Box<dyn MIDIInputObserver>>>>,

impl MIDIClient {
    pub fn new(name: &str, tx: std::sync::mpsc::Sender<u32>) -> Self {
        // let (tx, rx) = std::sync::mpsc::channel();

        let client = MIDIClientImpl::new(name, tx);
        let hash = hash(&client);
        let inner = std::sync::Arc::new(std::sync::Mutex::new(client));
        let clone = inner.clone();
        // let self_ = Self { inner, hash };

        // self_
        Self { inner, hash }
    }

    pub fn create_input_port(
        &self,
        name: &str,
        tx: std::sync::mpsc::Sender<MIDIPacket>,
        // f: impl Fn(crate::MIDIPacket),
    ) -> coremidi_sys::MIDIPortRef {
        self.inner.lock().unwrap().create_input_port(name, tx)
        // self.inner.lock().unwrap().create_input_port(name, f)
        // todo!()
    }

    pub fn create_output_port(&self, name: &str) -> coremidi_sys::MIDIPortRef {
        self.inner.lock().unwrap().create_output_port(name)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIClientImpl {
    inner: coremidi_sys::MIDIClientRef,
}

// type MIDIReceiveBlock = block::Block<(*const coremidi_sys::MIDIPacketList), ()>;

// coremidi_sys::MIDIReadBlock
impl MIDIClientImpl {
    // fn new(name: &str) -> Self {
    //     Self {
    //         inner: MIDIClientCreate(name, |x| { }),
    //     }
    // }

    fn notification(&mut self, u: u32) {}

    fn new(name: &str, tx: std::sync::mpsc::Sender<u32>) -> Self {
        Self {
            inner: MIDIClientCreate(name, tx),
        }
    }

    fn create_input_port(
        &self,
        name: &str,
        tx: std::sync::mpsc::Sender<MIDIPacket>,
        // f: impl Fn(crate::MIDIPacket),
    ) -> coremidi_sys::MIDIPortRef {
        let mut out = 0;
        let block = block::ConcreteBlock::new(move |packet: &coremidi_sys::MIDIPacketList| {
            let i = MIDIPacketListIterator::new(packet);
            for e in i {
                let _ = tx.send(MIDIPacket::from(e));
            }
        })
        .copy();

        unsafe {
            use core_foundation::base::TCFType;

            let name = core_foundation::string::CFString::new(name);
            os_assert(coremidi_sys::MIDIInputPortCreateWithBlock(
                self.inner,
                name.as_concrete_TypeRef(),
                &mut out,
                std::mem::transmute(block),
            ));
        }
        out
    }

    fn create_output_port(&self, name: &str) -> coremidi_sys::MIDIPortRef {
        let mut out = 0;
        let name = core_foundation::string::CFString::new(name);
        unsafe {
            // os_assert(coremidi_sys::MIDIOutputPortCreate(
            //     self.inner, name.as_mut_ptr(), &mut out,
            // ));
            todo!();
        }
        out
    }
}

impl std::hash::Hash for MIDIClientImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl PartialOrd for MIDIClientImpl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Drop for MIDIClientImpl {
    fn drop(&mut self) {
        unsafe {
            os_assert(coremidi_sys::MIDIClientDispose(self.inner));
        }
    }
}

fn MIDIClientCreate(name: &str, tx: std::sync::mpsc::Sender<u32>) -> coremidi_sys::MIDIClientRef {
    let mut out = 0;
    unsafe {
        use core_foundation::base::TCFType;
        let block = block::ConcreteBlock::new(move |notification: u32| {
            tx.send(10);
        })
        .copy();
        let name = core_foundation::string::CFString::new(name);
        os_assert(coremidi_sys::MIDIClientCreateWithBlock(
            name.as_concrete_TypeRef(),
            &mut out,
            std::mem::transmute(block),
        ));
    }
    out
}
