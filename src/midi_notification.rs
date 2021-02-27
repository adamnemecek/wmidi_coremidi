// use coremidi_sys::MIDINotificationMessageID;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MIDINotificationMessageID {
    SetupChanged = 1,
    ObjectAdded = 2,
    ObjectRemoved = 3,
    PropertyChanged = 4,
    ThruConnectionsChanged = 5,
    SerialPortOwnerChanged = 6,
    IOError = 7,
}

impl From<coremidi_sys::MIDINotificationMessageID> for MIDINotificationMessageID {
    fn from(a: coremidi_sys::MIDINotificationMessageID) -> Self {
        todo!()
        // match a {
        //     todo!()
        // }
    }
}

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
        // let msg = a.
        todo!()
    }
}
