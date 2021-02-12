use wmidi_coremidi::prelude::*;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let access = MIDIAccess::new("name", |notification| {
        tx.send(10);
    });

    let mut output = access.inputs().iter().find(|x|
        true
    );

    

    
}