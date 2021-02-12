use wmidi_coremidi::prelude::*;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let access = MIDIAccess::new("name", |notification| {
        tx.send(10);
    });

    let res = access.inputs().iter().find(|x|
        true
    );
    let input = res.unwrap().1;

    let mut input = input.clone();
    input.set_on_midi_message(|x| {

    });


    
}