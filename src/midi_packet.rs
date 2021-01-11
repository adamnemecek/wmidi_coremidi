const PACKET_DATA_SIZE: usize = 128;

// #[derive(Default)]
pub struct MIDIPacket {
    timestamp: u64,
    data: [u8; PACKET_DATA_SIZE],
}

impl Default for MIDIPacket {
    fn default() -> Self {
        Self {
            timestamp: 0,
            data: [0; 128],
        }
    }
}

impl MIDIPacket {
    // fn default() -> Self {
    //     todo!();
    // }
    // pub fn new() -> Self {
    //     todo!()
    // }

    pub(crate) fn from(ptr: &coremidi_sys::MIDIPacket) -> Self {
        assert!((ptr.length as usize) < PACKET_DATA_SIZE);
        let timestamp = ptr.timeStamp;

        let mut self_ = Self::default();
        // todo!()
        self_.timestamp = timestamp;
        let slice = unsafe { std::slice::from_raw_parts(&ptr.data as *const _, ptr.length as _) };
        self_.data.copy_from_slice(slice);
        self_
    }
}
