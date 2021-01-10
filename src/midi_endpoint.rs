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

impl MIDIEndpoint {
    fn id(&self) -> i64 {
        todo!()
    }

    fn flush(&self) {
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
        // MIDIObjectGetStringProperty(obj: u32, propertyID: *const __CFString, str: *mut *const __CFString) -> i32
        // coremidi_sys::MIDIObjectGetStringProperty(self.inner, propertyID, str)
        let s: *mut *const core_foundation::string::__CFString = std::ptr::null_mut();
        // coremidi_sys::MIDIObjectGetStringProperty(obj, property_id, &mut s);

        unsafe {
            // std::str::from_utf8_unchecked(&s.as_ref().unwrap())
        };
        todo!()
    }
}
