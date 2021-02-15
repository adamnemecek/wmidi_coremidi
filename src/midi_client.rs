use core_foundation::base::{
    OSStatus,
    TCFType,
};
// use coremidi_sys::{
//     MIDIClientRef,
//     MIDIReadBlock,
//     MIDIReadProc,
//     MIDITimeStamp,
// };

use crate::prelude::*;

#[derive(Debug)]
pub struct MIDIEvent<'a> {
    pub timestamp: coremidi_sys::MIDITimeStamp,
    pub data: &'a [u8],
}

impl<'a> MIDIEvent<'a> {
    pub fn new(timestamp: coremidi_sys::MIDITimeStamp, data: &'a [u8]) -> Self {
        Self { timestamp, data }
    }
}

impl<'a> From<&'a coremidi_sys::MIDIPacket> for MIDIEvent<'a> {
    fn from(p: &'a coremidi_sys::MIDIPacket) -> Self {
        let slice = unsafe { std::slice::from_raw_parts(p.data.as_ptr(), p.length as _) };

        Self::new(p.timeStamp, slice)
    }
}

// pub struct MIDIEventIterator<'a> {
//     p: &'a std::marker::PhantomData<()>,
// }

// impl<'a> Iterator for MIDIEventIterator<'a> {
//     type Item = MIDIEvent<'a>;
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
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

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct MIDIClient {
    inner: std::rc::Rc<MIDIClientImpl>,
}

impl MIDIClient {
    pub fn inner(&self) -> coremidi_sys::MIDIClientRef {
        self.inner.inner()
    }

    pub fn notification(&mut self, u: u32) {
        // self.inner.lock().unwrap().notification(u);
        todo!()
    }

    pub fn new(name: &str, f: impl Fn(MIDINotification) -> () + 'static) -> Self {
        let inner = MIDIClientImpl::new(name, f);
        Self {
            inner: inner.into(),
        }
    }

    // pub fn new(name: &str, tx: std::sync::mpsc::Sender<u32>) -> Self {
    //     todo!()
    //     // let (tx, rx) = std::sync::mpsc::channel();

    //     // let client = MIDIClientImpl::new(name, tx);
    //     // let hash = hash(&client);
    //     // let inner = std::sync::Arc::new(std::sync::Mutex::new(client));
    //     // // let clone = inner.clone();
    //     // let self_ = Self { inner, hash };
    //     // let mut clone = self_.clone();
    //     // std::thread::spawn(move || {
    //     //     let v = rx.recv().unwrap();
    //     //     clone.notification(v);
    //     // });

    //     // self_
    // }

    // pub(crate) fn create_input_port(
    //     &self,
    //     name: &str,
    //     // tx: std::sync::mpsc::Sender<MIDIPacket>,
    //     f: impl Fn(crate::MIDIPacket),
    // ) -> coremidi_sys::MIDIPortRef {
    //     // self.inner.lock().unwrap().create_input_port(name, tx)
    //     todo!()
    //     // self.inner.lock().unwrap().create_input_port(name, f)
    //     // todo!()
    // }

    // pub(crate) fn create_output_port(&self, name: &str) -> coremidi_sys::MIDIPortRef {
    //     self.inner.lock().unwrap().create_output_port(name)
    // }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct MIDIClientImpl {
    inner: coremidi_sys::MIDIClientRef,
}

impl MIDIClientImpl {
    pub(crate) fn inner(&self) -> coremidi_sys::MIDIClientRef {
        self.inner
    }
    fn notification(&mut self, u: u32) {}

    fn new(name: &str, f: impl Fn(MIDINotification) -> () + 'static) -> Self {
        Self {
            inner: midi_client_create(name, f),
        }
    }

    // fn create_input_port(
    //     &self,
    //     name: &str,
    //     // tx: std::sync::mpsc::Sender<MIDIPacket>,
    //     f: impl Fn(MIDIPacket) -> (),
    //     // f: impl Fn(crate::MIDIPacket),
    // ) -> coremidi_sys::MIDIPortRef {
    //     // typedef void (^MIDIReadBlock)(const MIDIPacketList *pktlist, void *srcConnRefCon);
    //     let mut out = 0;
    //     // let tx = tx.clone();
    //     let mut block = block::ConcreteBlock::new(
    //         move |packet: *const coremidi_sys::MIDIPacketList, _: *mut std::ffi::c_void| {
    //             // println!("input block");
    //             // todo!();
    //             // let i = MIDIPacketListIterator::new(unsafe { packet.as_ref().unwrap() });
    //             // let p = MIDIPacket::new(0, &[1,2,3]);
    //             // tx.send(p);
    //             // println!("here");
    //             // for e in i {
    //             //     let packet = MIDIPacket::from(e);
    //             //     let _ = tx.send(packet);
    //             // }
    //         },
    //     )
    //     .copy();

    //     let p = MIDIPacket::new(0, &[1, 2, 3]);
    //     // tx.send(p);
    //     // todo!()

    //     // let a =  F: FnMut(&PacketList) + Send + 'static
    //     // let f = |list: &coremidi_sys::MIDIPacketList| {

    //     // };
    //     // println!("creating input port {}", name);
    //     // let block_ref = & *block.copy();
    //     unsafe {
    //         use core_foundation::base::TCFType;

    //         let name = core_foundation::string::CFString::new(name);
    //         // MIDIInputPortCreateWithBlock(client, portName, outPort, readBlock)
    //         // os_assert(crate::MIDIInputPortCreateWithBlock(
    //         //     self.inner,
    //         //     name.as_concrete_TypeRef(),
    //         //     &mut out,
    //         //     // block_ref, // block,
    //         //     &block
    //         // ));

    //         // coremidi_sys::MIDIInputPortCreateWithBlock(client, portName, outPort, readBlock)

    //         // os_assert(coremidi_sys::MIDIInputPortCreateWithBlock(
    //         //     self.inner,
    //         //     name.as_concrete_TypeRef(),
    //         //     &mut out,
    //         //     // std::mem::transmute(block)
    //         //     block,
    //         // ));
    //         // coremidi_sys::MIDIInputPortCreateWithBlock(client, portName, outPort, readBlock)
    //         // let z: coremidi_sys::MIDIReadProc;
    //         // os_assert(coremidi_sys::MIDIInputPortCreateWithBlock(
    //         //     self.inner,
    //         //     name.as_concrete_TypeRef(),
    //         //     &mut out,
    //         //     // block as ,
    //         //     &*block as *const block::Block<_, _> as *mut std::ffi::c_void,
    //         // ));
    //         // extern "C" fn read_proc(
    //         //     pktlist: *const coremidi_sys::MIDIPacketList,
    //         //     read_proc_ref_con: *mut std::ffi::c_void,
    //         //     _src_conn_ref_con: *mut std::ffi::c_void,
    //         // ) {
    //         //     println!("readproc");
    //         // }

    //         // os_assert(coremidi_sys::MIDIInputPortCreate(
    //         //     self.inner,
    //         //     name.as_concrete_TypeRef(),
    //         //     // Some(Self::read_proc as extern "C" fn(_, _, _)),
    //         //     Some(read_proc),
    //         //     // box_callback.raw_ptr(),
    //         //     std::ptr::null_mut(),
    //         //     &mut out,
    //         // ));
    //     }
    //     assert!(out != 0);
    //     out
    // }

    // fn create_output_port(&self, name: &str) -> coremidi_sys::MIDIPortRef {
    //     let mut out = 0;
    //     unsafe {
    //         use core_foundation::base::TCFType;
    //         let name = core_foundation::string::CFString::new(name);
    //         os_assert(coremidi_sys::MIDIOutputPortCreate(
    //             self.inner,
    //             name.as_concrete_TypeRef(),
    //             &mut out,
    //         ));
    //     }
    //     out
    // }
}

// impl std::hash::Hash for MIDIClientImpl {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.inner.hash(state)
//     }
// }

// impl PartialOrd for MIDIClientImpl {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.inner.partial_cmp(&other.inner)
//     }
// }

impl Drop for MIDIClientImpl {
    fn drop(&mut self) {
        unsafe {
            os_assert(coremidi_sys::MIDIClientDispose(self.inner));
        }
    }
}

#[allow(improper_ctypes)]
extern "C" {
    pub fn MIDIClientCreateWithBlock(
        portName: *const core_foundation::string::__CFString,
        outClient: *mut coremidi_sys::MIDIClientRef,
        notifyBlock: block::RcBlock<(coremidi_sys::MIDINotification,), ()>,
    ) -> coremidi_sys::OSStatus;
}

#[allow(non_snake_case)]
fn midi_client_create(
    name: &str,
    f: impl Fn(MIDINotification) -> () + 'static,
) -> coremidi_sys::MIDIClientRef {
    let mut out = 0;
    unsafe {
        let block =
            block::ConcreteBlock::new(move |notification: coremidi_sys::MIDINotification| {
                if let Some(n) = MIDINotification::new(notification) {
                    f(n);
                }
            })
            .copy();
        let name = core_foundation::string::CFString::new(name);
        // os_assert(coremidi_sys::MIDIClientCreateWithBlock(
        //     name.as_concrete_TypeRef(),
        //     &mut out,
        //     std::mem::transmute(block),
        // ));
        os_assert(MIDIClientCreateWithBlock(
            name.as_concrete_TypeRef(),
            &mut out,
            block,
        ));
    }
    out
}
