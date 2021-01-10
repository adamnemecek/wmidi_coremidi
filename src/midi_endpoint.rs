// use core_foundation::string::__CFString;
// use coremidi_sys::MIDIEndpointRef;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct MIDIEndpoint {
    inner: coremidi_sys::MIDIEndpointRef,
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
    fn id(&self) -> i64 {
        todo!()
    }

    pub(crate) fn manufacturer(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyManufacturer) }
    }

    pub(crate) fn name(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyName) }
    }

    pub(crate) fn display_name(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyDisplayName) }
    }

    pub(crate) fn flush(&self) {
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
            __CFString,
            kCFStringEncodingUTF8,
            CFStringGetCStringPtr,
            CFStringGetLength,
        };
        let s: *mut *const __CFString = std::ptr::null_mut();

        unsafe {
            coremidi_sys::MIDIObjectGetStringProperty(self.inner, property_id, s);
            let len = CFStringGetLength(*s);
            let data = CFStringGetCStringPtr(*s, kCFStringEncodingUTF8) as *const u8;
            let slice = std::slice::from_raw_parts(data, len as _);
            std::str::from_utf8(slice).unwrap()
        }
    }
}
