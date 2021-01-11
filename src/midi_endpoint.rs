use crate::prelude::*;

// use core_foundation::string::__CFString;
// use coremidi_sys::MIDIEndpointRef;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MIDIEndpointKind {
    Device,
    Entity,
    Source,
    Destination,
    ExternalDevice,
    ExternalEntity,
    ExternalSource,
    ExternalDestination,
}

impl From<coremidi_sys::MIDIObjectType> for MIDIEndpointKind {
    fn from(a: coremidi_sys::MIDIObjectType) -> Self {
        match a {
            coremidi_sys::kMIDIObjectType_Device => Self::Device,
            coremidi_sys::kMIDIObjectType_Entity => Self::Entity,
            coremidi_sys::kMIDIObjectType_Source => Self::Source,
            coremidi_sys::kMIDIObjectType_Destination => Self::Destination,
            coremidi_sys::kMIDIObjectType_ExternalDevice => Self::ExternalDevice,
            coremidi_sys::kMIDIObjectType_ExternalEntity => Self::ExternalEntity,
            coremidi_sys::kMIDIObjectType_ExternalSource => Self::ExternalSource,
            coremidi_sys::kMIDIObjectType_ExternalDestination => Self::ExternalDestination,
            _ => todo!(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct MIDIEndpoint {
    inner: std::sync::Arc<MIDIEndpointImpl>,
}

impl MIDIEndpoint {
    pub fn new(inner: coremidi_sys::MIDIEndpointRef) -> Self {
        Self {
            inner: std::sync::Arc::new(MIDIEndpointImpl::new(inner)),
        }
    }

    pub fn flush(&self) {
        todo!()
    }

    pub fn id(&self) -> MIDIPortID {
        self.inner.id()
    }

    pub fn manufacturer(&self) -> &str {
        self.inner.manufacturer()
    }

    pub fn display_name(&self) -> &str {
        self.inner.display_name()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MIDIEndpointImpl {
    inner: coremidi_sys::MIDIEndpointRef,
}

impl MIDIEndpointImpl {
    fn new(inner: coremidi_sys::MIDIEndpointRef) -> Self {
        Self { inner }
    }
}

impl std::hash::Hash for MIDIEndpointImpl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl PartialOrd for MIDIEndpointImpl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Ord for MIDIEndpointImpl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl std::fmt::Debug for MIDIEndpointImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MIDIEndpoint {{{} {} {}}}",
            self.id(),
            self.display_name(),
            self.manufacturer()
        )
    }
}

impl MIDIEndpointImpl {
    fn id(&self) -> MIDIPortID {
        let id = unsafe { self.i32_property(coremidi_sys::kMIDIPropertyUniqueID) };

        MIDIPortID::new(id as _)
    }

    fn manufacturer(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyManufacturer) }
    }

    fn name(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyName) }
    }

    fn display_name(&self) -> &str {
        unsafe { self.str_property(coremidi_sys::kMIDIPropertyDisplayName) }
    }

    fn kind(&self) -> MIDIEndpointKind {
        let mut obj = 0;
        let mut kind = 0;

        unsafe {
            // coremidi_sys::MIDIObjectFindByUniqueID(self.inner)
            os_assert(coremidi_sys::MIDIObjectFindByUniqueID(
                self.id().inner,
                &mut obj,
                &mut kind,
            ));
        }
        kind.into()
    }

    fn version(&self) -> u32 {
        unsafe { self.i32_property(coremidi_sys::kMIDIPropertyDriverVersion) as _ }
    }

    fn state(&self) -> MIDIPortDeviceState {
        let v = unsafe { self.i32_property(coremidi_sys::kMIDIPropertyOffline) };
        if v == 0 {
            MIDIPortDeviceState::Connected
        } else {
            MIDIPortDeviceState::Disconnected
        }
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
