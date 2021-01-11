# wmidi_coremidi

Implementation of the `WebMIDI`.

Based on [WebMIDIKit](https://github.com/adamnemecek/WebMIDIKit).

<!-- 
# potentially useful rust features
* mpsc::receiver
* pinning?
-->

<!-- Uses coremidi-sys as opposed to coremidi since coremidi imposes -->
* use try_recv instead of getting the sender

* which is better api, create a receiver and have the user get it or have the user pass it in?

* the main problem with the current api is that when you 

The API design is inspired by the WebMIDI standard but diverges in some way. With WebMIDI, when you get a `MIDIInput`/`MIDIOutput`, you can immediately start sending and receiving data from/to it. In `wmidi`, you have to open the port and get a `MIDISender`/`MIDIReceiver` (based on `std::sync::mpsc::{Sender, Receiver}`). 

The reason for this departure is that `WebMIDI` uses JavaScript semantics where one can mutate objects without consideration for ownership. These semantics are a mismatch for `Rust`.

```rust
struct MIDIReceiver {
}
``` 