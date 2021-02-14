// #![feature(rustc_private)]
pub type DispatchQueue = *mut std::ffi::c_void;
pub type DispatchQueueAttr = *mut std::ffi::c_void;
// use block::Block;
// use std::libc::sleep;

use block::{
    Block,
    RcBlock,
};

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn dispatch_queue_create(label: *const i8, attr: DispatchQueueAttr) -> DispatchQueue;

    pub fn dispatch_async(queue: DispatchQueue, block: &Block<(), ()>);
}

fn main() {
    let queue = unsafe {
        dispatch_queue_create(
            b"fun::fun_capture::QuartzCapture\0".as_ptr() as *const _,
            std::ptr::null_mut(),
        )
    };

    // println!("hello");
    let dispatch_block = block::ConcreteBlock::new(move || {
        println!("dispatch block");
    });
    let dispatch_block = dispatch_block.copy();
    let dispatch_block: &Block<(), ()> = &dispatch_block;
    unsafe {
        dispatch_async(queue, dispatch_block);
    }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
