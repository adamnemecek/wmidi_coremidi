const PACKET_DATA_SIZE: usize = 256;

// #[derive(Default)]

pub(crate) struct MIDIPacket {
    pub(crate) timestamp: u64,
    pub(crate) len: usize,
    pub(crate) data: [u8; PACKET_DATA_SIZE],
}

impl Default for MIDIPacket {
    fn default() -> Self {
        Self {
            len: 0,
            timestamp: 0,
            data: [0; PACKET_DATA_SIZE],
        }
    }
}

impl std::fmt::Debug for MIDIPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MIDIPacket {{ timestamp: {}, data: {:?} }}",
            self.timestamp,
            self.as_slice()
        )
    }
}

// pub(crate) type MIDIReadBlock =
//     block::Block<(*const coremidi_sys::MIDIPacketList, *mut std::ffi::c_void), ()>;

// #[link(name = "CoreMIDI", kind = "framework")]
// extern "C" {
//     pub(crate) fn MIDIInputPortCreateWithBlock(
//         client: u32,
//         portName: *const core_foundation::string::__CFString,
//         outPort: *mut u32,
//         readBlock: MIDIReadBlock,
//         // read_block: *mut std::ffi:c_void
//     ) -> coremidi_sys::OSStatus;
// }

// pub fn current_host_time() -> u64 {
//     unsafe { AudioGetCurrentHostTime() }
// }

// pub fn convert_nanos_to_host_time(nanos: u64) -> u64 {
//     unsafe { AudioConvertNanosToHostTime(nanos) }
// }

impl MIDIPacket {
    // fn default() -> Self {
    //     todo!();
    // }
    // pub fn new() -> Self {
    //     todo!()
    // }

    pub fn new(timestamp: u64, data: &[u8]) -> Self {
        let mut d = [0; PACKET_DATA_SIZE];
        let len = data.len();
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), d.as_mut_ptr(), data.len());
        }
        Self {
            len,
            timestamp,
            data: d,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(&self.data as *const _, self.len) }
    }

    pub fn extend(&mut self, data: &[u8]) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.data[self.len..].as_mut_ptr(),
                data.len(),
            )
        }

        self.len += data.len();
    }

    pub(crate) fn from(ptr: &coremidi_sys::MIDIPacket) -> Self {
        assert!((ptr.length as usize) < PACKET_DATA_SIZE);
        let timestamp = ptr.timeStamp;
        // let z = coremidi_sys::AudioGetCurrentHostTime();
        let mut self_ = Self::default();
        // todo!()
        self_.timestamp = timestamp;
        let slice = unsafe { std::slice::from_raw_parts(&ptr.data as *const _, ptr.length as _) };
        self_.data.copy_from_slice(slice);
        self_
    }
}

impl From<&MIDIPacket> for coremidi_sys::MIDIPacket {
    fn from(a: &MIDIPacket) -> Self {
        Self {
            length: a.len() as _,
            data: a.data,
            timeStamp: a.timestamp,
            __padding: [0; 2],
        }
    }
}
