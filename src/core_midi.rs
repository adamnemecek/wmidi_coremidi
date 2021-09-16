

#[link(name = "CoreAudio", kind = "framework")]
extern "C" {
    fn AudioGetCurrentHostTime() -> u64;
    fn AudioConvertNanosToHostTime(nanos: u64) -> u64;
}

// pub type MIDITimeStamp = u64;

pub type UInt8 = ::std::os::raw::c_uchar;
pub type UInt16 = ::std::os::raw::c_ushort;
pub type UInt32 = ::std::os::raw::c_uint;
pub type SInt32 = ::std::os::raw::c_int;
pub type UInt64 = ::std::os::raw::c_ulonglong;
pub type OSStatus = SInt32;
pub type ByteCount = ::std::os::raw::c_ulong;
pub type ItemCount = ::std::os::raw::c_ulong;
pub type Boolean = ::std::os::raw::c_uchar;
pub type Byte = UInt8;


pub type MIDIObjectRef = UInt32;
pub type MIDIClientRef = MIDIObjectRef;
pub type MIDIPortRef = MIDIObjectRef;
pub type MIDIDeviceRef = MIDIObjectRef;
pub type MIDIEntityRef = MIDIObjectRef;
pub type MIDIEndpointRef = MIDIObjectRef;
pub type MIDITimeStamp = UInt64;
pub type MIDIObjectType = SInt32;


// #[repr()]
// enum MIDIObjectType {
//     // pub const kMIDIObjectType_Other: _bindgen_ty_65 = -1;
// // pub const kMIDIObjectType_Device: _bindgen_ty_65 = 0;
// // pub const kMIDIObjectType_Entity: _bindgen_ty_65 = 1;
// // pub const kMIDIObjectType_Source: _bindgen_ty_65 = 2;
// // pub const kMIDIObjectType_Destination: _bindgen_ty_65 = 3;
// // pub const kMIDIObjectType_ExternalDevice: _bindgen_ty_65 = 16;
// // pub const kMIDIObjectType_ExternalEntity: _bindgen_ty_65 = 17;
// // pub const kMIDIObjectType_ExternalSource: _bindgen_ty_65 = 18;
// // pub const kMIDIObjectType_ExternalDestination: _bindgen_ty_65 = 19;
// }

#[link(name = "CoreMIDI", kind = "framework")]
extern "C" {

    //     ./midi_port_map.rs:        let count = unsafe { coremidi_sys::MIDIGetNumberOfSources() } as _;
    pub fn MIDIGetNumberOfSources() -> ItemCount;

    // ./midi_port_map.rs:                let endpoint = coremidi_sys::MIDIGetSource(i as _);
    pub fn MIDIGetSource(sourceIndex0: ItemCount) -> MIDIEndpointRef;

    // ./midi_port_map.rs:        let count = unsafe { coremidi_sys::MIDIGetNumberOfDestinations() } as _;
    pub fn MIDIGetNumberOfDestinations() -> ItemCount;

    // ./midi_port_map.rs:                let endpoint = coremidi_sys::MIDIGetDestination(i as _);
    pub fn MIDIGetDestination(sourceIndex0: ItemCount) -> MIDIEndpointRef;

    // ./midi_packet.rs://     block::Block<(*const coremidi_sys::MIDIPacketList, *mut std::ffi::c_void), ()>;

    // ./midi_packet.rs://     ) -> coremidi_sys::OSStatus;

    // ./midi_packet.rs:    pub(crate) fn from(ptr: &coremidi_sys::MIDIPacket) -> Self {

    // ./midi_packet.rs:        // let z = coremidi_sys::AudioGetCurrentHostTime();

    // ./midi_packet_list.rs:            // let packet = unsafe { &*(self.inner as *const coremidi_sys::MIDIPacket) };

    // ./midi_packet_list.rs:            self.inner = unsafe { coremidi_sys::MIDIPacketNext(self.inner) };
    fn MIDIPacketNext();

    // ./midi_output.rs:    client_ref: coremidi_sys::MIDIClientRef,

    // ./midi_output.rs:) -> coremidi_sys::MIDIPortRef {

    // ./midi_output.rs:        os_assert(coremidi_sys::MIDIOutputPortCreate(
    fn MIDIOutputPortCreate(/* */) -> OSStatus;

    // ./midi_output.rs:            os_assert(coremidi_sys::MIDISend(
    fn MIDISend(/* */) -> OSStatus;

    // ./midi_output.rs:            os_assert(coremidi_sys::MIDIReceived(
    fn MIDIReceived(/* */) -> OSStatus;

    // ./midi_sender.rs:// fn CreateMIDIPacket(timestamp: u64, data: &[u8]) -> coremidi_sys::MIDIPacket {

    // ./midi_sender.rs://     let mut self_ = coremidi_sys::MIDIPacket {

    // ./midi_sender.rs:// fn CreateMIDIPacketList(timestamp: u64, data: &[u8]) -> coremidi_sys::MIDIPacketList {

    // ./midi_sender.rs://     coremidi_sys::MIDIPacketList {

    // ./midi_sender.rs://         port: coremidi_sys::MIDIPortRef,

    // ./midi_input.rs:            os_assert(coremidi_sys::MIDIPortConnectSource(
    fn MIDIPortConnectSource() -> OSStatus;

    // ./midi_input.rs:            os_assert(coremidi_sys::MIDIPortDisconnectSource(
    fn MIDIPortDisconnectSource() -> OSStatus;

    // ./midi_input.rs:    block::Block<(*const coremidi_sys::MIDIPacketList, *mut std::ffi::c_void), ()>;

    // ./midi_input.rs:        move |packet: *const coremidi_sys::MIDIPacketList, _: *mut std::ffi::c_void| {

    // ./midi_client.rs:    pub timestamp: coremidi_sys::MIDITimeStamp,

    // ./midi_client.rs:    pub fn new(timestamp: coremidi_sys::MIDITimeStamp, data: &'a [u8]) -> Self {

    // ./midi_client.rs:impl<'a> From<&'a coremidi_sys::MIDIPacket> for MIDIEvent<'a> {

    // ./midi_client.rs:    fn from(p: &'a coremidi_sys::MIDIPacket) -> Self {

    // ./midi_client.rs:    pub fn inner(&self) -> coremidi_sys::MIDIClientRef {

    // ./midi_client.rs:    // ) -> coremidi_sys::MIDIPortRef {

    // ./midi_client.rs:    // pub(crate) fn create_output_port(&self, name: &str) -> coremidi_sys::MIDIPortRef {

    // ./midi_client.rs:    //         // coremidi_sys::MIDIInputPortCreateWithBlock(client, portName, outPort, readBlock)
    fn MIDIInputPortCreateWithBlock() -> OSStatus;

    // ./midi_client.rs:    //         os_assert(coremidi_sys::MIDIOutputPortCreate(

    // ./midi_client.rs:            os_assert(coremidi_sys::MIDIClientDispose(self.inner));
    fn MIDIClientDispose() -> OSStatus;

    // ./midi_client.rs:        notifyBlock: block::RcBlock<(coremidi_sys::MIDINotification,), ()>,

    // ./midi_client.rs:    ) -> coremidi_sys::OSStatus;

    // ./midi_client.rs:) -> coremidi_sys::MIDIClientRef {

    // ./midi_client.rs:            block::ConcreteBlock::new(move |notification: coremidi_sys::MIDINotification| {

    // ./midi_client.rs:        // os_assert(coremidi_sys::MIDIClientCreateWithBlock(

    // ./midi_notification.rs:// use coremidi_sys::MIDINotificationMessageID;

    // ./midi_notification.rs:impl From<coremidi_sys::MIDINotificationMessageID> for MIDINotificationMessageID {

    // ./midi_notification.rs:    fn from(a: coremidi_sys::MIDINotificationMessageID) -> Self {

    // ./midi_notification.rs:// impl From<coremidi_sys::MIDINotification> for Option<MIDINotification> {

    // ./midi_notification.rs://     fn from (a: coremidi_sys::MIDINotification) -> Self {

    // ./midi_notification.rs:    pub fn new(a: coremidi_sys::MIDINotification) -> Option<Self> {

    // ./midi_endpoint.rs:impl From<coremidi_sys::MIDIObjectType> for MIDIEndpointKind {

    // ./midi_endpoint.rs:    fn from(a: coremidi_sys::MIDIObjectType) -> Self {

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_Device => Self::Device,

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_Entity => Self::Entity,

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_Source => Self::Source,

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_Destination => Self::Destination,

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_ExternalDevice => Self::ExternalDevice,

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_ExternalEntity => Self::ExternalEntity,

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_ExternalSource => Self::ExternalSource,

    // ./midi_endpoint.rs:            coremidi_sys::kMIDIObjectType_ExternalDestination => Self::ExternalDestination,

    // ./midi_endpoint.rs://             Self::UniqueID => coremidi_sys::kMIDIPropertyUniqueID,

    // ./midi_endpoint.rs://             Self::Version => coremidi_sys::kMIDIPropertyDriverVersion,

    // ./midi_endpoint.rs://             Self::Manufacturer => coremidi_sys::kMIDIPropertyManufacturer,

    // ./midi_endpoint.rs:        let id = unsafe { self.i32_property(coremidi_sys::kMIDIPropertyUniqueID) };

    // ./midi_endpoint.rs:        unsafe { self.str_property(coremidi_sys::kMIDIPropertyManufacturer) }

    // ./midi_endpoint.rs:        unsafe { self.str_property(coremidi_sys::kMIDIPropertyName) }

    // ./midi_endpoint.rs:        unsafe { self.str_property(coremidi_sys::kMIDIPropertyDisplayName) }

    // ./midi_endpoint.rs:            os_assert(coremidi_sys::MIDIObjectFindByUniqueID(
    fn MIDIObjectFindByUniqueID();

    // ./midi_endpoint.rs:        unsafe { self.i32_property(coremidi_sys::kMIDIPropertyDriverVersion) as _ }

    // ./midi_endpoint.rs:        let v = unsafe { self.i32_property(coremidi_sys::kMIDIPropertyOffline) };

    // ./midi_endpoint.rs:            coremidi_sys::MIDIFlushOutput(self.inner);
    fn MIDIFlushOutput();

    // ./midi_endpoint.rs:            coremidi_sys::MIDIObjectGetIntegerProperty(self.inner, property_id, &mut out);
    fn MIDIObjectGetIntegerProperty();

    // ./midi_endpoint.rs:            coremidi_sys::MIDIObjectGetStringProperty(self.inner, property_id, s.as_mut_ptr());
    fn MIDIObjectGetStringProperty();

    // ./midi_endpoint.rs:fn MIDIObjectGetType(id: coremidi_sys::MIDIEndpointRef) -> MIDIPortKind {
    fn MIDIObjectGetType();

}
