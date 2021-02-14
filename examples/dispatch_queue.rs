
pub type DispatchQueue = *mut std::ffi::c_void;
pub type DispatchQueueAttr = *mut std::ffi::c_void;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn dispatch_queue_create(
        label: *const i8,
        attr: DispatchQueueAttr,
      ) -> DispatchQueue;
}

fn main() {

    println!("hello");

}