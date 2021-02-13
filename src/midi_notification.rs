pub enum MIDINotification {
    Added(),
    Removed(),
}

// impl From<coremidi_sys::MIDINotification> for Option<MIDINotification> {
//     fn from (a: coremidi_sys::MIDINotification) -> Self {

//         todo!()
//     }
// }

impl MIDINotification {
    pub fn new(a: coremidi_sys::MIDINotification) -> Option<Self> {
        todo!()
    }
}
