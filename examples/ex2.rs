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
    // let (_, output) = outputs.first().unwrap();
    // output.
    for (_, p) in outputs.iter() {
        println!("{:?}", p);
    }

    let outputs = outputs
        .iter()
        .find(|x| x.1.display_name().contains("Driver"));
    let mut output = outputs.unwrap().1.clone();

    let input = access.input_for(&output);
    output.send(&[0x90, 100, 100], None);

    // for (i, e) in outputs.iter() {
    //     println!("{:?}", e);
    // }

    // let input = res.unwrap().1;

    // let mut input = input.clone();
    // input.set_on_midi_message(|x| {
    //     tx.send(10);
    // });
}
