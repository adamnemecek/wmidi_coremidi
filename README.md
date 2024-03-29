# wmidi_coremidi: Simplest Rust MIDI library
`wmidi_coremidi` is an implementation of the `WebMIDI` standard for Apple devices.

<!--
```/Users/adamnemecek/.cargo/registry/src/github.com-1ecc6299db9ec823/coremidi-sys-2.0.2/src```

https://github.com/waddlesplash/QMidi/blob/master/src/OS/QMidi_CoreMidi.cpp

## block
https://github.com/suzusuzu/virtualization-rs/blob/9d58132ef5b63ce403be292ea580d767f6083407/examples/simplevm.rs#L130


-->

## About
# 
### What's MIDI

[MIDI](https://en.wikipedia.org/wiki/MIDI) is a standard governing music software and music device interconnectivity. It lets you make music by sending data between applications and devices.

### What's WebMIDI

[WebMIDI](https://webaudio.github.io/web-midi-api/) is a browser API standard that brings the MIDI technology to the web. WebMIDI is minimal, it only describes MIDI port selection, receiving data from input ports and sending data to output ports. [WebMIDI is currently implemented in Chrome & Opera](http://caniuse.com/#feat=midi). Note that WebMIDI is relatively low level as messages are still represented as sequences of u8's (bytes/octets).

### What's wmidi
wmidi is an implementation of the WebMIDI API for macOS/iOS. On these OS, the native framework for working with MIDI is [CoreMIDI](https://developer.apple.com/reference/coremidi).

The implementation is based on [`WebMIDIKit`](https://github.com/adamnemecek/WebMIDIKit), a framework with the same API in Swift.

CoreMIDI is old and the API is entirely in C (💩). Using it involves a lot of void pointer casting (💩^9.329), and other unspeakable things. Furthermore, some of the APIs didn't quite survive the transition to Swift and are essentially unusable in Swift (`MIDIPacketList` APIs, I'm looking at you).

CoreMIDI is also extremely verbose and error prone. Selecting an input port and receiving data from it is __~80 lines__ of [convoluted Swift code](http://mattg411.com/coremidi-swift-programming/). __wmidi let's you do the same thing in 1.__

## Usage

### Selecting an input port and receiving MIDI messages from it

```rust
use wmidi_coremidi::prelude::*;

/// represents the MIDI session
let access = MIDIAccess::new();

/// prints all MIDI inputs available to the console and asks the user which port they want to select
let inputPort: Option<&MIDIInput> = access.inputs().prompt();

/// Receiving MIDI events
/// set the input port's onMIDIMessage callback which gets called when the port receives MIDI packets
inputPort.unwrap().set_on_midi_message(std::rc::Rc::new(|x: MIDIEvent<'_>| {

}))

```


### Selecting an output port and sending MIDI packets to it
```rust
/// select an output port
let outputPort: MIDIOutput? = midi.outputs.prompt()

/// send messages to it
outputPort.map {

	/// send a note on message
	/// the bytes are in the normal MIDI message format (https://www.midi.org/specifications/item/table-1-summary-of-midi-message)
	/// i.e. you have to send two events, a note on event and a note off event to play a single note
	/// the format is as follows:
	/// byte0 = message type (0x90 = note on, 0x80 = note off)
	/// byte1 = the note played (0x60 = C8, see http://www.midimountain.com/midi/midi_note_numbers.html)
	/// byte2 = velocity (how loud the note should be 127 (=0x7f) is max, 0 is min)

	let noteOn: [UInt8] = [0x90, 0x60, 0x7f]
	let noteOff: [UInt8] = [0x80, 0x60, 0]

	/// send the note on event
	$0.send(noteOn)

	/// send a note off message 1000 ms (1 second) later
	$0.send(noteOff, offset: 1000)

	/// in WebMIDIKit, you can also chain these
	$0.send(noteOn)
	  .send(noteOff, offset: 1000)
}
```

If the output port you want to select has a corresponding input port you can also do

```rust
let outputPort: MIDIOutput? = midi.output(for: inputPort)
```

Similarly, you can find an input port for the output port.

```rust
let inputPort2: MIDIInput? = midi.input(for: outputPort)
```

### Looping over ports

Port maps are dictionary like collections of `MIDIInputs` or `MIDIOutputs` that are indexed with the port's id. As a result, you cannot index into them like you would into an array (the reason for this being that the endpoints can be added and removed so you cannot reference them by their index).
```rust
for (id, port) in midi.inputs {
	print(id, port)
}
```



## Installation

Use Swift Package Manager. Add the following `.Package` entry into your dependencies.

```rust
.Package(url: "https://github.com/adamnemecek/webmidikit", from: "1.0.0")
```

 If you are having any build issues, look at the sample project [sample project](https://github.com/adamnemecek/WebMIDIKitDemo).

## Documentation

### MIDIAccess
Represents the MIDI session. See [spec](https://www.w3.org/TR/webmidi/#midiaccess-interface).

```rust
struct MIDIAccess {
	/// collections of MIDIInputs and MIDIOutputs currently connected to the computer
	fn inputs() MIDIInputMap { get }
	var outputs: MIDIOutputMap { get }

	/// will be called if a port changes either connection state or port state
	var onStateChange: ((MIDIPort) -> ())? = nil { get set }

	init()

	/// given an output, tries to find the corresponding input port
	func input(for port: MIDIOutput) -> MIDIInput?

	/// given an input, tries to find the corresponding output port
	/// if you send data to the output port returned, the corresponding input port
	/// will receive it (assuming the `onMIDIMessage` is set)
	func output(for port: MIDIInput) -> MIDIOutput?
}
```

### MIDIPort

See [spec](https://www.w3.org/TR/webmidi/#midiport-interface). Represents the base class of `MIDIInput` and `MIDIOutput`.

Note that you don't construct MIDIPorts nor it's subclasses yourself, you only get them from the `MIDIAccess` object. Also note that you only ever deal with subclasses or `MIDIPort` (`MIDIInput` or `MIDIOutput`) never `MIDIPort` itself.

```rust
pub trait MIDIPort: Eq + Clone + std::hash::Hash + std::fmt::Debug {
    fn id(&self) -> MIDIPortID;
    fn manufacturer(&self) -> &str;
    fn name(&self) -> &str;
    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind;

    fn display_name(&self) -> &str;
    fn connection(&self) -> MIDIPortConnectionState;
    fn open(&mut self);
    fn close(&mut self);

    fn on_state_change(&self) -> Option<StateChangeFn<Self>>;
    fn set_on_state_change(&mut self, on_state_change: Option<StateChangeFn<Self>>);
}

// struct MIDIPort {

// 	fn id() -> ;
// 	var manufacturer: String { get }

// 	var name: String { get }

// 	/// .input (for MIDIInput) or .output (for MIDIOutput)
// 	var type: MIDIPortType { get }

// 	var version: Int { get }

// 	/// .connected | .disconnected,
// 	/// indicates if the port's endpoint is connected or not
// 	var state: MIDIPortDeviceState { get }

// 	/// .open | .closed
// 	var connection: MIDIPortConnectionState { get }

// 	/// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
// 	func open()

// 	/// closes the port
// 	func close()
// }
```

### MIDIInput

Allows for receiving data send to the port.

See [spec](https://www.w3.org/TR/webmidi/#midiinput-interface).

```rust
struct MIDIInput: MIDIPort {
	/// set this and it will get called when the port receives messages.
	var onMIDIMessage: ((MIDIPacket) -> ())? = nil { get set }
}
```


### MIDIOutput


See [spec](https://www.w3.org/TR/webmidi/#midioutput-interface).
```rust
struct MIDIOutput: MIDIPort {

	/// send data to port, note that unlike the WebMIDI API,
	/// the last parameter specifies offset from now, when the event should be scheduled (as opposed to absolute timestamp)
	/// the unit remains milliseconds though.
	/// note that right now, WebMIDIKit doesn't support sending multiple packets in the same call, to send multiple packets, you need on call per packet
	fn send(_ data: &[u8], offset: TimeStamp);

	// clear all scheduled but yet undelivered midi events
	fn clear();
}
```




