const PACKET_DATA_SIZE: usize = 256;

// #[derive(Default)]
pub struct MIDIPacket {
    timestamp: u64,
    len: usize,
    data: [u8; PACKET_DATA_SIZE],
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

#[link(name = "CoreAudio", kind = "framework")]
extern "C" {
    fn AudioGetCurrentHostTime() -> u64;
    fn AudioConvertNanosToHostTime(nanos: u64) -> u64;
}

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
        d.copy_from_slice(data);
        Self {
            len,
            timestamp,
            data: d,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn slice(&self) -> &[u8] {
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
