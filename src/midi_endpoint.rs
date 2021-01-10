use crate::prelude::*;

// use core_foundation::string::__CFString;
// use coremidi_sys::MIDIEndpointRef;

#[derive(Clone, PartialEq, Eq)]
pub struct MIDIEndpoint {
    inner: coremidi_sys::MIDIEndpointRef,
}

impl MIDIEndpoint {
    pub fn new(inner: coremidi_sys::MIDIEndpointRef) -> Self {
        Self { inner }
    }
}

impl std::hash::Hash for MIDIEndpoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl PartialOrd for MIDIEndpoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Ord for MIDIEndpoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl MIDIEndpoint {
    pub fn id(&self) -> i32 {
        unsafe { self.i32_property(coremidi_sys::kMIDIPropertyUniqueID) }
    }

    pub fn manufacturer(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyManufacturer) }
    }

    pub fn name(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyName) }
    }

    pub fn display_name(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyDisplayName) }
    }

    pub fn kind(&self) -> MIDIPortKind {
        // return MIDIPortType(MIDIObjectGetType(id: id))
        // unsafe { coremidi_sys::MIDIObjectGetTy }
        todo!()
    }

    pub fn version(&self) -> u32 {
        unsafe { self.i32_property(coremidi_sys::kMIDIPropertyDriverVersion) as _ }
    }

    pub fn state(&self) -> MIDIPortDeviceState {
        let v = unsafe { self.i32_property(coremidi_sys::kMIDIPropertyOffline) };
        if v == 0 {
            MIDIPortDeviceState::Connected
        } else {
            MIDIPortDeviceState::Disconnected
        }
    }

    pub fn flush(&self) {
        unsafe {
            coremidi_sys::MIDIFlushOutput(self.inner);
        }
    }

    fn i32_property(&self, property_id: *const core_foundation::string::__CFString) -> i32 {
        let mut out = 0;
        unsafe {
            coremidi_sys::MIDIObjectGetIntegerProperty(self.inner, property_id, &mut out);
        }
        out
    }

    fn str_property(&self, property_id: *const core_foundation::string::__CFString) -> &str {
        use core_foundation::string::{
            kCFStringEncodingUTF8,
            CFStringGetCStringPtr,
            CFStringGetLength,
        };
        let mut s = std::mem::MaybeUninit::uninit();

        unsafe {
            coremidi_sys::MIDIObjectGetStringProperty(self.inner, property_id, s.as_mut_ptr());
            let s = s.assume_init();
            let len = CFStringGetLength(s);
            let data = CFStringGetCStringPtr(s, kCFStringEncodingUTF8) as *const u8;
            let slice = std::slice::from_raw_parts(data, len as _);
            std::str::from_utf8(slice).unwrap()
        }
    }
}


fn MIDIObjectGetType(id: coremidi_sys::MIDIEndpointRef) -> MIDIPortKind {
    // let object = std::mem::MaybeUninit::uninit();
    let kind = coremidi_sys::kMIDIObjectType_Other;
    // let kind = coremidi_sys::MIDIObjectType::Other;
    use coremidi_sys::{
        MIDIObjectFindByUniqueID,
        MIDIUniqueID,
    };

    // os_assert(MIDIObjectFindByUniqueID(id, &mut object, &mut kind));

    // os_assert(MIDIObjectFindByUniqueID(MIDIUniqueID(id), &object, &kind));
    // return type
    match kind {
        _ => todo!(),
    }
}
