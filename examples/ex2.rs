use wmidi_coremidi::prelude::*;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let midi_access = MIDIAccess::new("name", |notification| {
        tx.send(10);
    });
}