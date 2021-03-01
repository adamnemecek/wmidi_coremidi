// #![feature(rustc_private)]
use block::{
    Block,
    RcBlock,
};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DispatchQueue(*mut std::ffi::c_void);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DispatchQueueAttr(*mut std::ffi::c_void);

// use block::Block;
// use std::libc::sleep;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn dispatch_queue_create(label: *const i8, attr: DispatchQueueAttr) -> DispatchQueue;
    fn dispatch_async(queue: DispatchQueue, block: &Block<(), ()>);
}

impl DispatchQueue {
    pub fn new(label: &str) -> Self {
        unsafe {
            dispatch_queue_create(
                b"label\0".as_ptr() as *const _,
                DispatchQueueAttr(std::ptr::null_mut()),
            )
        }
    }

    pub fn dispatch_async(&self, f: impl Fn() -> () + 'static) {
        let block = block::ConcreteBlock::new(move || {
            f()
        }).copy();
        unsafe {
            dispatch_async(*self, &block);
        }
    }
}

fn main() {

    let queue = DispatchQueue::new("label");
    queue.dispatch_async(|| {
        println!("async dispatched");
    });
    
    // let queue = unsafe {
    //     dispatch_queue_create(
    //         b"fun::fun_capture::QuartzCapture\0".as_ptr() as *const _,
    //         std::ptr::null_mut(),
    //     )
    // };

    // // println!("hello");
    // let dispatch_block = block::ConcreteBlock::new(move || {
    //     println!("dispatch block");
    // });
    // let dispatch_block = dispatch_block.copy();
    // let dispatch_block: &Block<(), ()> = &dispatch_block;
    // unsafe {
    //     dispatch_async(queue, dispatch_block);
    // }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
