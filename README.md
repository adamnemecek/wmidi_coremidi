# wmidi_coremidi

```/Users/adamnemecek/.cargo/registry/src/github.com-1ecc6299db9ec823/coremidi-sys-2.0.2/src```

## block
https://github.com/suzusuzu/virtualization-rs/blob/9d58132ef5b63ce403be292ea580d767f6083407/examples/simplevm.rs#L130



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