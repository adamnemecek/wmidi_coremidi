use wmidi_coremidi::prelude::*;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let access = MIDIAccess::new("name", move |notification| {
        tx.send(10);
    });

    // let res = access.inputs().iter().find(|x| true);
    // let mut input;
    // for (i, e) in access.inputs().iter() {
    //     input = e;
    //     println!("{:?} {:?}", i, e);
    // }

    let outputs: Vec<_> = access.outputs().iter().collect();
    let (_, output) = outputs.first().unwrap();

    // let input = res.unwrap().1;

    // let mut input = input.clone();
    // input.set_on_midi_message(|x| {
    //     tx.send(10);
    // });
}
